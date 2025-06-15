#[cfg(test)]
mod test {
    #[test]
    pub fn miri() {
        let mut data = 10;
        let ref1 = &mut data;
        let ref2 = &mut *ref1;

        *ref2 += 1;
        *ref1 += 2;

        println!("updated data {}", data);

        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr = ref1 as *mut _;

            // ORDER SWAPPED
            *ref1 += 1;
            *ptr += 2;

            println!("updated data {}", data);
        }

        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;

            // Access the first raw pointer first
            *ptr2 += 2;

            // Then access things in the order of the 'borrows stack'
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;

            println!("updated data {}", data);
        }
        
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;

            // Then access things in the order of the 'borrows stack'
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;

            println!("updated data {}", data);
        }
        
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_1 = ptr2_at_0.add(1);
            
            *ptr3_at_1 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;
            
            println!("{:?}", &data[..]);
        }
        
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_0 = ptr2_at_0;
            
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;
            
            println!("{:?}", &data[..]);
        }
        
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr2_at_0 = ref1_at_0 as *mut i32;
            let ptr3_at_0 = ptr2_at_0;
            let ptr4_at_0 = ptr3_at_0.add(0);
            let ptr5_at_0 = ptr4_at_0.add(1).sub(1);
            
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ptr4_at_0 += 4;
            *ptr5_at_0 += 5;
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;
            
            println!("{:?}", &data[..]);
        }
        
        // unsafe {
        //     let mut data = [0; 10];
        //     let ref1_at_0 = &mut data[0];
        //     let ref2_at_1 = &mut data[1];
        //     let ptr3_at_0 = ref1_at_0 as *mut i32;
        //     let ptr4_at_0 = ref2_at_1 as *mut i32;
            
        //     *ptr4_at_0 += 4;
        //     *ptr3_at_0 += 3;
        //     *ref2_at_1 += 2;
        //     *ref1_at_0 += 1;
            
        //     println!("{:?}", &data[..]);
        // }
        
        unsafe {
            let mut data = [0; 10];
            let slice1 = &mut data[..];
            let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);
            
            let ref4_at_0 = &mut slice2_at_0[0];
            let ref5_at_1 = &mut slice3_at_1[0];
            
            let ptr6_at_0 = ref4_at_0 as *mut i32;
            let ptr7_at_1 = ref5_at_1 as *mut i32;
            
            *ptr7_at_1 += 7;
            *ptr6_at_0 += 6;
            *ref5_at_1 += 5;
            *ref4_at_0 += 4;
            
            println!("{:?}", &data[..]);
        }
        
        unsafe {
            let mut data = [0; 10];
            let slice_all = &mut data[..];
            
            let ptr2_all = slice_all.as_mut_ptr();
            
            let ptr3_at_0 = ptr2_all;
            let ptr4_at_1 = ptr2_all.add(1);
            let ref5_at_0 = &mut *ptr3_at_0;
            let ref6_at_1 = &mut *ptr4_at_1;
            
            *ref6_at_1 += 6;
            *ref5_at_0 += 5;
            *ptr4_at_1 += 4;
            *ptr3_at_0 += 3;
            
            for idx in 0..10 {
                *ptr2_all.add(idx) += idx;
            }
            
            for (idx, elem_ref) in slice_all.iter_mut().enumerate() {
                *elem_ref += idx;
            }
            
            println!("{:?}", &data[..])
        }
    }
}
