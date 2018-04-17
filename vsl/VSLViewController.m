#import "VSLViewController.h"

extern "C" {
    VSLViewController* VSLViewControllerInit(VSLViewControllerVTable* vtable) {
        return [[VSLViewController alloc]initFromVTable: vtable];
    }

    UIStoryboard* VSLViewControllerGetStoryboard(VSLViewController* vc) {
        return vc.storyboard;
    }

    UIView* VSLViewControllerGetView(VSLViewController* vc) {
        return vc.view;
    }
}

@interface VSLViewController () {
    VSLViewControllerVTable* vtable;
}

@end

@implementation VSLViewController

- (instancetype)initFromVTable:(VSLViewControllerVTable*)vtableRef {
    if ((self = [super init])) {
        vtable = vtableRef;
    }
    return self;
}

- (void)viewDidLoad {
    [super viewDidLoad];
    vtable->viewDidLoad(self);
}


- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    vtable->didReceiveMemoryWarning(self);
}


@end
