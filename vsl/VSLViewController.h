#import <UIKit/UIKit.h>

@class VSLViewController;

typedef struct {
    void(*viewDidLoad)(VSLViewController*);
    void(*didReceiveMemoryWarning)(VSLViewController*);
} VSLViewControllerVTable;

@interface VSLViewController : UIViewController
- (instancetype)initFromVTable:(VSLViewControllerVTable*)vtable;
@end
