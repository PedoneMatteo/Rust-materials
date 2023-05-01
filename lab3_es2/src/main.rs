#[derive(Clone)]

struct MYCycle<I: Clone + Iterator>{
    base_iter: I,
    current: Option<I>,
    repeat: usize,
    counter: usize,
}
//MyCycle::new(iter: I, repeat: usize);
impl <I: Clone + Iterator> MYCycle<I> {

    fn new(iter: I, repeat: usize) -> Self {
        Self {
            base_iter: iter.clone(),
            current: Some(iter),
            repeat,
            counter: 0,
        }
    }
}

impl <I: Clone+Iterator> Iterator for MYCycle<I>{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeat > 0 && self.counter >= self.repeat {
            // Se abbiamo ripetuto abbastanza volte, l'iteratore finisce
            return None;
        }
        let next_item = self.current.as_mut().unwrap().next();

        if next_item.is_none() {
            self.current=Some(self.base_iter.clone());
            self.counter+=1;
            return self.next();
        }

        next_item
    }
}


fn main() {
    let v0:Vec<i32> = Vec::new();    //vettore vuoto
    let v1 = vec![0,1,2];
    let v2 = vec![3,4,5,6];

    let my_cycle0 = MYCycle::new(v0.into_iter(), 2);    //ITERATORE CON 0 ELEMENTI


    let  my_cycle1= MYCycle::new(v1.into_iter(), 2);
    let  my_cycle2= MYCycle::new(v2.into_iter(), 2);

    let my_cycle3 = MYCycle::new(my_cycle1.clone(), 3);
    for el in my_cycle3 {
        println!("{:?}", el);
    }
    println!("------------------");

    let c = my_cycle1.clone().chain(my_cycle2.clone());
    for el in c {
        println!("{:?}", el);
    }
    println!("------------------");

    let z = my_cycle1.zip(my_cycle2);
    z.into_iter().for_each(|el| println!("{:?}", el));

}
