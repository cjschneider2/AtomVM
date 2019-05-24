/***************************************************************************
 *   Copyright 2018 by Riccardo Binetti <rbino@gmx.com>                    *
 *                                                                         *
 *   This program is free software; you can redistribute it and/or modify  *
 *   it under the terms of the GNU Lesser General Public License as        *
 *   published by the Free Software Foundation; either version 2 of the    *
 *   License, or (at your option) any later version.                       *
 *                                                                         *
 *   This program is distributed in the hope that it will be useful,       *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of        *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
 *   GNU General Public License for more details.                          *
 *                                                                         *
 *   You should have received a copy of the GNU General Public License     *
 *   along with this program; if not, write to the                         *
 *   Free Software Foundation, Inc.,                                       *
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA .        *
 ***************************************************************************/

#include <errno.h>
#include <stdio.h>
#include <unistd.h>
#include <libopencm3/cm3/nvic.h>
#include <libopencm3/cm3/systick.h>
#include <libopencm3/stm32/gpio.h>
#include <libopencm3/stm32/rcc.h>
#include <libopencm3/stm32/usart.h>

#include <avmpack.h>
#include <context.h>
#include <globalcontext.h>
#include <module.h>

#define USART_CONSOLE USART2
#define AVM_ADDRESS (0x8080000)
#define AVM_FLASH_MAX_SIZE (0x80000)
#define CLOCK_FREQUENCY (168000000)

int _write(int file, char *ptr, int len);

static void clock_setup()
{
    // Use external clock, set divider for 168 MHz clock frequency
    rcc_clock_setup_hse_3v3(&rcc_hse_8mhz_3v3[RCC_CLOCK_3V3_168MHZ]);

    // Enable clock for USART2 GPIO
    rcc_periph_clock_enable(RCC_GPIOA);

    // Enable clock for USART2
    rcc_periph_clock_enable(RCC_USART2);
}

static void usart_setup()
{
    // Setup GPIO pins for USART2 transmit
    gpio_mode_setup(GPIOA, GPIO_MODE_AF, GPIO_PUPD_NONE, GPIO2);

    // Setup USART2 TX pin as alternate function
    gpio_set_af(GPIOA, GPIO_AF7, GPIO2);

    usart_set_baudrate(USART_CONSOLE, 115200);
    usart_set_databits(USART_CONSOLE, 8);
    usart_set_stopbits(USART_CONSOLE, USART_STOPBITS_1);
    usart_set_mode(USART_CONSOLE, USART_MODE_TX);
    usart_set_parity(USART_CONSOLE, USART_PARITY_NONE);
    usart_set_flow_control(USART_CONSOLE, USART_FLOWCONTROL_NONE);

    // Finally enable the USART
    usart_enable(USART_CONSOLE);
}

// Set up a timer to create 1ms ticks
// The handler is in sys.c
static void systick_setup()
{
    // clock rate / 1000 to get 1ms interrupt rate
    systick_set_reload(CLOCK_FREQUENCY / 1000);
    systick_set_clocksource(STK_CSR_CLKSOURCE_AHB);
    systick_counter_enable();
    systick_interrupt_enable();
}

// Use USART_CONSOLE as a console.
// This is a syscall for newlib
int _write(int file, char *ptr, int len)
{
    int i;

    if (file == STDOUT_FILENO || file == STDERR_FILENO) {
        for (i = 0; i < len; i++) {
            if (ptr[i] == '\n') {
                usart_send_blocking(USART_CONSOLE, '\r');
            }
            usart_send_blocking(USART_CONSOLE, ptr[i]);
        }
        return i;
    }
    errno = EIO;
    return -1;
}

int main()
{
    clock_setup();
    systick_setup();
    usart_setup();

    const void *flashed_avm = (void *) AVM_ADDRESS;
    uint32_t size = AVM_FLASH_MAX_SIZE;

    uint32_t startup_beam_size;
    const void *startup_beam;
    const char *startup_module_name;

    printf("Booting file mapped at: %p, size: %li\n", flashed_avm, size);

    GlobalContext *glb = globalcontext_new();

    if (!avmpack_is_valid(flashed_avm, size) || !avmpack_find_section_by_flag(flashed_avm, BEAM_START_FLAG, &startup_beam, &startup_beam_size, &startup_module_name)) {
        fprintf(stderr, "error: invalid AVM Pack\n");
        return 1;
    }

    glb->avmpack_data = flashed_avm;
    glb->avmpack_platform_data = NULL;

    Module *mod = module_new_from_iff_binary(glb, startup_beam, startup_beam_size);
    globalcontext_insert_module_with_filename(glb, mod, startup_module_name);
    Context *ctx = context_new(glb);

    printf("Starting: %s...\n", startup_module_name);
    printf("---\n");
    context_execute_loop(ctx, mod, "start", 0);
    printf("Return value: %lx\n", (long) term_to_int32(ctx->x[0]));

    while (1);

    return 0;
}
