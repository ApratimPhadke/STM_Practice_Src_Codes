<div align="center">
  <h1>🚀 The Bare-Metal Embedded C Bible</h1>
  <p><i>A comprehensive guide and code repository for mastering standard C and bare-metal programming on STM32 microcontrollers.</i></p>
</div>

---

## 📖 About This Repository
Welcome to the Bare-Metal Embedded C Bible! This repository is engineered to bridge the gap between basic standard C programming and advanced hardware-level embedded systems development. 

Instead of relying heavily on hardware abstraction layers (HAL), these practice codes dive deep into register-level manipulation, memory architecture, and core processor logic. It is structured into host-machine C exercises and target-specific STM32 firmware implementations.

---

## 🗂️ Repository Structure

The repository is divided into three main environments: Host code, Target code, and Experimental code.

### 💻 1. `host/` (Core C Fundamentals)
These folders contain standard C programs meant to be compiled and executed on your host PC. They build the foundation required for writing efficient embedded code.

* **Arrays & Strings:** `arrays`, `arrays_passing`, `arrays_swap`, `strings`
* **Pointers & Memory:** `pointer`, `pointer_1`, `pointer_2`, `var_address`, `function`
* **Bitwise Operations (Crucial):** `bitWise2Numbers`, `bitWiseEvenOdd`, `bitWiseSet`
* **Structs, Unions & Memory Alignment:** `structInit`, `struct_array`, `structPointer`, `structPacket`, `structPacketBitfield`, `union`, `unionBitExraction`, `dataAlignment`
* **Control Flow & Math:** `Average`, `math`, `forLoopEvenNum`, `WhileLoop1to10`, `switchCaseArea`, `ifelseBiggestOfTwo`
* **Embedded Specifics:** `constdata`, `macro`, `type_casting`

### 🎯 2. `target/stm32f407_disc/` (STM32 Bare-Metal Firmware)
These projects are pure bare-metal C applications intended to be flashed onto an STM32 microcontroller. While the folder is named for the F407 Discovery, the logic is highly portable.

* `001HelloWorld` & `002Sizeof` - Basic setup and ITM debugging.
* `003Add` - Arithmetic operations verifying processor ALUs.
* `004led_on` & `005led_toggle` - Direct GPIO Register manipulation (ODR, MODER).
* `006pin_read` - Reading input states from GPIO pins (IDR).
* `007volatile` - Demonstrating the necessity of the `volatile` keyword in hardware access.
* `008button_ISR` - Configuring NVIC and EXTI lines for hardware interrupts.
* `009packed_Vsnonpacked` - Memory padding analysis on the ARM Cortex-M architecture.
* `010led_toggle_bitfields` - Using C structs and bitfields to map hardware registers cleanly.
* `011keypad` - Matrix keypad interfacing using raw GPIO state machines.

### 🦀 3. `Rust/`
* `gussing_game` & `practice` - Experimental workspace for exploring safe systems programming using Rust and Cargo.

---

## 🧠 Essential Bare-Metal Theory
To truly utilize this repository, you must understand a few core concepts of embedded C:

### 1. Bitwise Operations
In bare-metal programming, you rarely write whole bytes to a register. You manipulate specific bits without altering others.
* **Setting a bit:** `Register |= (1 << bit_position);`
* **Clearing a bit:** `Register &= ~(1 << bit_position);`
* **Toggling a bit:** `Register ^= (1 << bit_position);`

### 2. The `volatile` Keyword (`007volatile`)
Hardware registers can change state without the software's knowledge (e.g., an input pin changes from low to high). If you don't declare a hardware pointer as `volatile`, the compiler's optimizer might cache the value and ignore real-time hardware changes. It tells the compiler: *"Always fetch this value directly from memory, never optimize it away."*

### 3. Structs and Bitfields (`010led_toggle_bitfields`)
Instead of using raw hex addresses and bitwise logic, you can define a C `struct` that perfectly maps to a peripheral's memory layout. Bitfields (`uint32_t pin0 : 1;`) allow you to access single bits seamlessly: `GPIOA->ODR.pin5 = 1;`

### 4. Memory Alignment (`009packed_Vsnonpacked`)
ARM Cortex-M processors prefer fetching data aligned to 32-bit word boundaries. The compiler automatically adds "padding" bytes to structs to ensure this. In embedded systems, when transmitting structs over UART or SPI, this padding corrupts data packets. You must use compiler attributes like `__attribute__((packed))` to strip this padding.

---

## 🛠️ Hardware Requirements

This code is written at the bare-metal level, meaning the underlying C concepts are completely hardware-agnostic. You can port and run these projects on virtually **any STM32 development board**:

* 💊 **STM32 Bluepill** (STM32F103C8T6) 
* 💊 **STM32 Blackpill** (STM32F401CCU6 / STM32F411CEU6) 
* 🛡️ **STM32 Nucleo Boards** (e.g., Nucleo-F446RE)
* 🧭 **STM32 Discovery Boards** (e.g., STM32F407G-DISC1)
* *Custom custom-designed PCB.*

**Required Programmer:**
* **ST-Link V2** (USB Dongle) - Essential if using a Bluepill/Blackpill. 
    * *Wiring:* `SWDIO` -> `SWDIO`, `SWCLK` -> `SWCLK`, `GND` -> `GND`, `3.3V` -> `3.3V`.
* *(Nucleo and Discovery boards have an ST-Link integrated right into the board!)*

---

## 🚀 How to Setup, Extract, and Flash

We recommend using **STM32CubeIDE** (ST's official, free, Eclipse-based IDE) to compile and debug these codes.

### Step 1: Download the Repository
1. Click the **`Code`** button at the top right of this page and select **`Download ZIP`**.
2. Extract the `.zip` file to your preferred local directory.

### Step 2: Prepare STM32CubeIDE
1. Open **STM32CubeIDE**.
2. Go to `File` -> `New` -> `STM32 Project`.
3. Select your exact Microcontroller or Development Board in the Target Selector.
4. Name your project. 
5. When prompted to initialize all peripherals with default settings, you can choose **No** (we are writing bare-metal code, so we don't necessarily need the HAL auto-generated files, though keeping them as a reference is fine).

### Step 3: Import and Run the Code
1. Navigate to your extracted repository folder.
2. Find the project you want to test (e.g., `target/stm32f407_disc/004led_on`).
3. Copy the contents of the `Src` folder (usually `main.c`).
4. In STM32CubeIDE, paste and overwrite the `main.c` file inside your project's `Core/Src/` directory.
5. **Important Note on Porting:** If you are using a board other than the STM32F407, you will need to open the `main.c` file and update the base memory addresses (e.g., `RCC_BASE`, `GPIOA_BASE`) by referencing the Reference Manual for your specific MCU.

### Step 4: Build and Flash
1. Click the 🔨 **Build (Hammer)** icon to compile the code. Ensure there are 0 errors.
2. Connect your STM32 board to your PC (via ST-Link).
3. Click the 🐞 **Debug (Bug)** icon. This flashes the code to the MCU and halts the processor.
4. Click the **Resume (Play)** button, or step through the code line-by-line while inspecting peripheral registers in the IDE's SFR (Special Function Register) view!

---
*Maintained by Apratim Phadke*
