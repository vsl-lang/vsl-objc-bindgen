import UIKit

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?


    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        return AppDelegateDidFinishLaunchingWithOptions(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self),
            unsafeBitCast(launchOptions, to: UnsafeMutableRawPointer.self)
        )
    }

    func applicationWillResignActive(_ application: UIApplication) {
        AppDelegateWillResignActive(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self)
        )
    }

    func applicationDidEnterBackground(_ application: UIApplication) {
        AppDelegateDidEnterBackground(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self)
        )
    }

    func applicationWillEnterForeground(_ application: UIApplication) {
        AppDelegateDidEnterBackground(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self)
        )
    }

    func applicationDidBecomeActive(_ application: UIApplication) {
        AppDelegateDidBecomeActive(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self)
        )
    }

    func applicationWillTerminate(_ application: UIApplication) {
        AppDelegateWillTerminate(
            unsafeBitCast(application, to: UnsafeMutableRawPointer.self)
        )
    }


}

