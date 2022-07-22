trait FlyBehaviour {
    fn fly(&self);
}
struct FlyWithWings;

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("I can fly!")
    }
}

struct FlyWithNoWings;
impl FlyBehaviour for FlyWithNoWings {
    fn fly(&self) {
        println!("I can't fly!")
    }
}

trait Duck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour;
    fn fly(&self) {
        let s = self.get_fly_behaviour();
        s.fly();
    }
}

struct MallardDuck {
    fly_behaviour: Box<dyn FlyBehaviour>,
}

impl Duck for MallardDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl MallardDuck {
    fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> Self {
        MallardDuck { fly_behaviour }
    }

    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour;
    }
}

struct ModelDuck {
    fly_behaviour: Box<dyn FlyBehaviour>,
}

impl Duck for ModelDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl ModelDuck {
    fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> Self {
        ModelDuck { fly_behaviour }
    }
}

fn main() {
    let mut mallard_duck = MallardDuck::new(Box::new(FlyWithWings));
    mallard_duck.fly();
    mallard_duck.set_fly_behaviour(Box::new(FlyWithNoWings));
    mallard_duck.fly();

    let model_duck = ModelDuck::new(Box::new(FlyWithNoWings));
    model_duck.fly();
}
