@delegate
public class AppDelegate {
    @delegate(DidFinishLaunching) public func didFinishLaunching(application: UIApplication, with options: SwiftDictionary) {}

    @delegate(WillResignActive) public func willResignActive(application: UIApplication) {}
    @delegate(DidEnterBackground) public func didEnterBackground(application: UIApplication) {}
    @delegate(WillEnterBackground) public func willEnterBackground(application: UIApplication) {}

    @delegate(DidBecomeActive) public func didBecomeActive(application: UIApplication) {}
    @delegate(WillTerminate) public func willTerminate(application: UIApplication) {}
}
