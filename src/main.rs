#![allow(dead_code)] // FIXME remove when port is near done
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

fn main() {
    println!("Hello, World!!");
}

// oops.h

//
//  oops.h
//  Smalltalk-80
//
//  Created by Dan Banay on 3/5/20;
//  Copyright © 2020 Dan Banay; All rights reserved;
//
//  MIT License
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
//

// #pragma once

// // Well known oops for various objects in an image

// // initializeGuaranteedPointers  G&R pg. 576

// // UndefinedObject and Booleans
// static const int NilPointer = 2;
// static const int FalsePointer = 4;
// static const int TruePointer = 6;

type ConstType = i16;

const NilPointer: ConstType = 2;
const FalsePointer: ConstType = 4;
const TruePointer: ConstType = 7;

// // Root
// static const int SchedulerAssociationPointer = 8;
// static const int SmalltalkPointer = 25286; // SystemDictionary

const SchedulerAssociationPointer: ConstType = 8;
const SmalltalkPointer: ConstType = 25286; // SystemDictionary

// // Classes

// static const int ClassSmallInteger = 12;
// static const int ClassStringPointer = 14;
// static const int ClassArrayPointer = 16;
// static const int ClassMethodContextPointer = 22;
// static const int ClassBlockContextPointer = 24;
// static const int ClassPointPointer = 26;
// static const int ClassLargePositiveIntegerPointer = 28;
// static const int ClassMessagePointer = 32;
// static const int ClassCharacterPointer = 40;
// static const int ClassCompiledMethod = 34;
// static const int ClassSymbolPointer = 56;

// static const int ClassFloatPointer = 20;

// static const int ClassSemaphorePointer = 38;
// static const int ClassDisplayScreenPointer = 834;
// static const int ClassUndefinedObject = 25728;

const ClassSmallInteger: ConstType = 12;
const ClassStringPointer: ConstType = 14;
const ClassArrayPointer: ConstType = 16;
const ClassMethodContextPointer: ConstType = 22;
const ClassBlockContextPointer: ConstType = 24;
const ClassPointPointer: ConstType = 26;
const ClassLargePositiveIntegerPointer: ConstType = 28;
const ClassMessagePointer: ConstType = 32;
const ClassCharacterPointer: ConstType = 40;
const ClassCompiledMethod: ConstType = 34;
const ClassSymbolPointer: ConstType = 56;

const ClassFloatPointer: ConstType = 20;

const ClassSemaphorePointer: ConstType = 38;
const ClassDisplayScreenPointer: ConstType = 834;
const ClassUndefinedObject: ConstType = 25728;

// // Selectors
// static const int DoesNotUnderstandSelector = 42;
// static const int CannotReturnSelector = 44;
// static const int MustBeBooleanSelector = 52;

const DoesNotUnderstandSelector: ConstType = 42;
const CannotReturnSelector: ConstType = 44;
const MustBeBooleanSelector: ConstType = 52;

// // Tables
// static const int SpecialSelectorsPointer = 48;
// static const int CharacterTablePointer = 50;

const SpecialSelectorsPointer: ConstType = 48;
const CharacterTablePointer: ConstType = 50;

// /*
//  dbanay - first oops 2..52 are special oops... see this from SystemTracer:
//  If using GC make sure these are roots
//  specialObjects _
//       "1:" (Array with: nil with: false with: true with: (Smalltalk associationAt: #Processor))
//      , "5:" (Array with: Symbol table with: SmallInteger with: String with: Array)
//      , "9:" (Array with: (Smalltalk associationAt: #Smalltalk) with: Float
//                  with: MethodContext with: BlockContext)
//      , "13:" (Array with: Point with: LargePositiveInteger with: DisplayBitmap with: Message)
//      , "17:" (Array with: CompiledMethod with: #unusedOop18 with: Semaphore with: Character)
//      , "21:" (Array with: #doesNotUnderstand: with: #cannotReturn:
//                  with: #monitor: with: Smalltalk specialSelectors)
//      , "25:" (Array with: Character characterTable with: #mustBeBoolean).
//  specialObjects size = 26 ifFalse: [self error: 'try again!!'].
//  */

//
//  hal.h
//  Smalltalk-80
//
//  Created by Dan Banay on 4/13/20.
//  Copyright © 2020 Dan Banay. All rights reserved.
//
//  MIT License
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
//

//
//  hal.h
//  Smalltalk-80
//
//  Created by Dan Banay on 4/13/20.
//  Copyright © 2020 Dan Banay. All rights reserved.
//
//  MIT License
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
//

// #pragma once
// #include <cstdint>

// class IHardwareAbstractionLayer
// {
// public:
//     // Specify the semaphore to signal on input
//     virtual void set_input_semaphore(int semaphore) = 0;
//     // The number of seconds since 00:00 in the morning of January 1, 1901
//     virtual std::uint32_t get_smalltalk_epoch_time() = 0;
//     // the number of milliseconds since the millisecond clock was
//     // last reset or rolled over (a 32-bit unsigned number)
//     virtual std::uint32_t get_msclock() = 0;
//     // Schedule a semaphore to be signaled at a time. Only one outstanding
//     // request may be scheduled at anytime. When called any outstanding
//     // request will be replaced (or canceled if semaphore is 0).
//     // Will signal immediate if scheduled time has passed.
//     virtual void signal_at(int semaphore, std::uint32_t msClockTime) = 0;
//     // Set the cursor image
//     // (a 16 word form)
//     virtual void set_cursor_image(std::uint16_t *image) = 0;
//     // Set the mouse cursor location
//     virtual void set_cursor_location(int x, int y) = 0;
//     virtual void get_cursor_location(int *x, int *y) = 0;
//     virtual void set_link_cursor(bool link) = 0;
//     // Set the display size
//     virtual bool set_display_size(int width, int height) = 0;
//     // Notify that screen contents changed
//     virtual void display_changed(int x, int y, int width, int height) = 0;
//     // Input queue
//     virtual bool next_input_word(std::uint16_t *word) = 0;
//     // Report catastrophic failure
//     virtual void error(const char *message) = 0;
//     // Lifetime
//     virtual void signal_quit() = 0;
//     virtual void exit_to_debugger() = 0;
//     // Snapshot name
//     virtual const char *get_image_name() = 0;
//     virtual void set_image_name(const char *new_name) = 0;
// };

type int = i16;

pub trait IHardwareAbstractionLayer {
    // Specify the semaphore to signal on input
    fn set_input_semaphore(semaphore: int);
    // The number of seconds since 00:00 in the morning of January 1, 1901
    fn get_smalltalk_epoch_time() -> u32;
    // the number of milliseconds since the millisecond clock was
    // last reset or rolled over (a 32-bit unsigned number)
    fn get_msclock() -> u32;
    // Schedule a semaphore to be signaled at a time. Only one outstanding
    // request may be scheduled at anytime. When called any outstanding
    // request will be replaced (or canceled if semaphore is 0).
    // Will signal immediate if scheduled time has passed.
    fn signal_at(semaphore: int, ms_clock_time: u32);
    // Set the cursor image
    // (a 16 word form)
    fn set_cursor_image(image: &[u16]);
    // Set the mouse cursor location
    fn set_cursor_location(x: int, y: int);
    fn get_cursor_location() -> (int, int);
    fn set_link_cursor(link: bool);
    // Set the display size
    fn set_display_size(width: int, height: int) -> bool;
    // Notify that screen contents changed
    fn display_changed(x: int, y: int, width: int, height: int);
    // Input queue
    fn next_input_word(word: &[u16]) -> bool;
    // Report catastrophic failure
    fn error(message: String);
    // Lifetime
    fn signal_quit();
    fn exit_to_debugger();
    // Snapshot name
    fn get_image_name() -> String;
    fn set_image_name(new_name: String);
}

