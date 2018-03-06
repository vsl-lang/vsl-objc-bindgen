//
//  vsl-ios.h
//  vsl-ios
//
//  Created by Vihan Bhargava on 3/4/18.
//  Copyright © 2018 VSL. All rights reserved.
//

#ifndef vsl_ios_h
#define vsl_ios_h

#include <stdbool.h>

#define SWIFT_CLASS(name) Swift ## name
#define DECLARE_SWIFT_CLASS(name) typedef void* SWIFT_CLASS(name)
#define DECLARE_METHOD(head, args, name) head name args
#define ARGLIST( ... ) __VA_ARGS__

DECLARE_SWIFT_CLASS(OpaquePointer);

DECLARE_SWIFT_CLASS(UIApplication);
DECLARE_SWIFT_CLASS(Dictionary);

#include "AppDelegate.h"

#endif /* vsl_ios_h */
