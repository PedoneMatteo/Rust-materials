
pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    buffer: /*Vec<T>, */Vec<Option<T>>,         //per rispondere alla seconda domanda dell'esercitazione si usa la Option nel caso in cui si torna un NULL
    size: usize,
    read_index: usize,
    write_index: usize
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {

    pub fn new(capacity: usize) -> Self {
        CircularBuffer{
            buffer: (0..capacity).map(|_| None).collect(),  //vec![T::default(),capacity],
            size: capacity,
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buffer[self.write_index].is_some(){
            return Err(Error::FullBuffer);
        }
        self.buffer[self.write_index] = Some(_element); //la some mi fa avere un Option con element all'interno
        self.write_index = ( self.write_index + 1 ) % self.size;
        Ok(())

    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.buffer[self.read_index].take(){
            Some(value) => {
                self.read_index = (self.read_index + 1) % self.size;
                Ok(value)
            }
            None => Err(Error::EmptyBuffer),
        }
   /*     self.buffer[self.read_index]
            .take()
            .ok_or(Error::EmptyBuffer)
            .map(|value| {
                self.read_index= ( self.read_index + 1 ) % self.size;
                value
            })  */
    }

    pub fn clear(&mut self) {
        self.buffer = (0..self.size).map(|_| None).collect();  //vec![T::default(),capacity],
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        let full = self.buffer[self.write_index].is_some(); //controllo se il vettore è full
        self.buffer[self.write_index] = Some(_element);
        self.write_index= ( self.write_index + 1 ) % self.size;
        self.read_index=self.write_index;
        if full{
            self.read_index = self.write_index;
        }
    }
}

fn main(){

}