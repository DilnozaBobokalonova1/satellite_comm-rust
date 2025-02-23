These are my own notes as developing.
For the button configuration, i enabled a pull-up resistor to be utilized
for the pin's default state. Reasoning:
✅ Pull-Up Configuration 
The button connects PA0 to GND when pressed.
When not pressed, PA0 "floats" unless pulled up internally.
We enable the internal pull-up resistor, so PA0 reads HIGH (1) by default.
When the button is pressed, PA0 gets connected to GND and reads LOW (0).
Why it’s useful? ✔ Simple wiring: Only need a button and a ground connection.
✔ Default state = HIGH, meaning less risk of accidental triggers from floating inputs.

🚫 Pull-Down Configuration (Alternative)
The button connects PA0 to VCC (3.3V) when pressed.
When not pressed, PA0 "floats" unless pulled down internally.
We enable the internal pull-down resistor, so PA0 reads LOW (0) by default.
When the button is pressed, PA0 gets connected to 3.3V and reads HIGH (1).
Why I didn’t use this? The STM32 has built-in pull-up resistors, so pull-ups are more commonly used.
Also, Pull-down configurations usually require an external resistor to avoid weak pull-down behavior.

EXTICR Register	            Manages
    EXTICR1	        EXTI0, EXTI1, EXTI2, EXTI3
    EXTICR2	        EXTI4, EXTI5, EXTI6, EXTI7
    EXTICR3	        EXTI8, EXTI9, EXTI10, EXTI11
    EXTICR4	        EXTI12, EXTI13, EXTI14, EXTI15

Pin	EXTI Line
PA0	EXTI0
PA1	EXTI1
PA2	EXTI2
PA3	EXTI3
and so on

1️⃣ PA0 (Button Pin) is configured as an input with a pull-up resistor.
2️⃣ EXTI0 (External Interrupt 0) is configured for PA0.
3️⃣ Falling-edge detection is enabled, so it triggers when PA0 goes from HIGH → LOW (button press).
4️⃣ NVIC is unmasked (NVIC::unmask(pac::Interrupt::EXTI0);) to enable the interrupt.
5️⃣ When PA0 is pressed, the CPU jumps to the EXTI0 handler.
6️⃣ The handler clears the EXTI flag and sends a UART message.

If my UART settings use 8-N-1 (8 data bits, No parity, 1 stop bit), then:
Each byte (character) = 8 data bits + 1 start bit + 1 stop bit
Total bits per character = 10 bits
Effective bytes per second = 115,200 / 10 = 11,520 bytes per second (Bps)
That’s 11.52 KB/s of raw data transfer.

On Interrupts:
✔ PR1 is used to track and clear EXTI0-EXTI31 interrupts.
✔ We use pr0.set_bit() because EXTI0 (PA0) corresponds to bit 0.
✔ Writing 1 clears the flag to prevent repeated interrupts.
✔ PR2 exists in some chips but isn't used for EXTI0-EXTI31 on STM32L4.