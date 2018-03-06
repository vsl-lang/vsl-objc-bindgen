//
//  AppDelegate.h
//
//
//  Created by Vihan Bhargava on 3/5/18.
//

#ifndef AppDelegate_h
#define AppDelegate_h

#include "vsl-ios.h"

#define AppDelegate(head, args, name) DECLARE_METHOD(head, args, AppDelegate ## name)
#define AppDelegateListener(name) AppDelegate(void, (SwiftUIApplication application), name)

AppDelegate(bool, (SwiftUIApplication application, SwiftDictionary options), DidFinishLaunchingWithOptions);
AppDelegateListener(WillResignActive);
AppDelegateListener(DidEnterBackground);
AppDelegateListener(WillEnterForeground);
AppDelegateListener(DidBecomeActive);
AppDelegateListener(WillTerminate);

#endif /* AppDelegate_h */
