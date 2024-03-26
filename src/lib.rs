#![allow(non_snake_case)]

use Matrix::Matrix;
use num_complex::Complex;

pub trait Binary<T>{
    fn new(bynary: Vec<i32>) -> Result<Self, &'static str> where Self: Sized;
    fn get_state(&self) -> &Vec<T>;
    fn set_state(self,state: i32, index: usize) -> Result<(), &'static str>;
    
}
pub struct QuBinary{
    state: Vec<Matrix<Complex<f64>>>
}
impl Binary<Matrix<Complex<f64>>> for QuBinary{ 
    fn new(binary: Vec<i32>) -> Result<Self, &'static str>{
        let mut v: Vec<Matrix<Complex<f64>>> = vec![Matrix::new(2, 1, Complex::<f64>::new(0.0, 0.0)); binary.len()];

        for i in 0..binary.len(){
            if binary[i] == 0{
                v[i] = <QuBinary as SetOneZero::<Matrix<Complex<f64>>>>::set_zero();
            }else if binary[i] == 1{
                v[i] = <QuBinary as SetOneZero::<Matrix<Complex<f64>>>>::set_one();
            }else {
                return Err("Contains number other than 0 or 1");
            }
        }
        Ok(QuBinary{state: v})
    }
    fn get_state(&self) -> &Vec<Matrix<Complex<f64>>>{
        &self.state
    }
    fn set_state(mut self,state: i32, index: usize) -> Result<(), &'static str>{
        if self.state.len() < index + 1{
            return  Err("Index out of bounds")
        }else if state == 0{
            self.state[index] = <QuBinary as SetOneZero::<Matrix<Complex<f64>>>>::set_zero();
            Ok(())
        }else if state == 1{
            self.state[index] = <QuBinary as SetOneZero::<Matrix<Complex<f64>>>>::set_one();
            Ok(())
        }else{
            return Err("Contains number other than 0 or 1");
        }
    }
}

impl Clone for QuBinary {
    fn clone(&self) -> QuBinary{
        QuBinary{state: self.state.clone()}
    }
} 

pub struct NomarlBinary<T>{
    state: Vec<T>
}
impl Binary<i32> for NomarlBinary<i32>{
    fn new(binary: Vec<i32>) -> Result<Self, &'static str>{
        let mut v: Vec<i32> = vec![0; binary.len()];
        for i in 0..binary.len(){
            if binary[i] == 0{
                v[i] = <NomarlBinary<i32> as SetOneZero::<i32>>::set_zero();
            }else if binary[i] == 1{
                v[i] = <NomarlBinary<i32> as SetOneZero::<i32>>::set_one();
            }else {
                return Err("Contains number other than 0 or 1");
            }
        }
        Ok(NomarlBinary{state: v})
    }
    fn get_state(&self) -> &Vec<i32>{
        &self.state
    }
    fn set_state(mut self,state: i32, index: usize) -> Result<(), &'static str>{
        if self.state.len() < index + 1{
            return  Err("Index out of bounds")
        }else if state == 0{
            self.state[index] = <NomarlBinary<i32> as SetOneZero::<i32>>::set_zero();
            Ok(())
        }else if state == 1{
            self.state[index] = <NomarlBinary<i32> as SetOneZero::<i32>>::set_one();
            Ok(())
        }else{
            return Err("Contains number other than 0 or 1");
        }
    }
}

trait SetOneZero<T>{
    fn set_zero() -> T;
    fn set_one() -> T;
}
impl SetOneZero<Matrix<Complex<f64>>> for QuBinary{
    fn set_zero() -> Matrix<Complex<f64>>{
        let v: Vec<Vec<Complex<f64>>> = vec![vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]];
        let m: Matrix<Complex<f64>> = Matrix::new(2, 1, Complex::<f64>::new(0.0, 0.0));
        m.set_state(v, 2, 1).unwrap()
    }
    fn set_one() -> Matrix<Complex<f64>>{
        let v: Vec<Vec<Complex<f64>>> = vec![vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]];
        let m: Matrix<Complex<f64>> = Matrix::new(2, 1, Complex::<f64>::new(0.0, 0.0));
        m.set_state(v, 2, 1).unwrap()
    }
}
impl SetOneZero<i32> for NomarlBinary<i32>{
    fn set_zero() -> i32{
        0 as i32
    }
    fn set_one() -> i32{
        1 as i32
    }
}

