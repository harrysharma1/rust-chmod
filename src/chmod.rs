use std::collections::HashMap;



const BINDING: &str = "()";
#[allow(dead_code)]
pub struct Chmod{

}


impl Chmod{
    fn map_octal(&self,key:u16)->String{
        let mut octal_to_symbolic:HashMap<u16,String> = HashMap::new(); 
        octal_to_symbolic.insert(0, "---".to_string());
        octal_to_symbolic.insert(1, "--x".to_string());
        octal_to_symbolic.insert(2, "-w-".to_string());
        octal_to_symbolic.insert(3, "-wx".to_string());
        octal_to_symbolic.insert(4, "r--".to_string());
        octal_to_symbolic.insert(5, "r-x".to_string());
        octal_to_symbolic.insert(6, "rw-".to_string());
        octal_to_symbolic.insert(7, "rwx".to_string());

        let binding = BINDING.to_string();

        octal_to_symbolic.get(&key).unwrap_or(&binding.to_string()).to_string()
    }
    
    pub fn convert_octal_to_symbolic(&self,octal:u16)->String{

        if  octal>999{
            let error_incorrect_length = "<Err: Not correct length>\n".to_string();
            return error_incorrect_length;      
        }

        let user:u16  = (octal/100)%10;
        let group:u16 = (octal/10)%10;
        let owner:u16 = octal % 10;

        self.octal_error_check(user);
        self.octal_error_check(group);
        self.octal_error_check(owner);

        let user_string = self.map_octal(user);
        let group_string = self.map_octal(group);
        let owner_string = self.map_octal(owner);
        let full_string = format!("{}{}{}\n",user_string,group_string,owner_string);
        full_string

    }

    fn map_symbolic(&self,key:String)->String{
        let mut symbolic_to_octal: HashMap<String, u16> = HashMap::new();
        symbolic_to_octal.insert("---".to_string(), 0);
        symbolic_to_octal.insert("--x".to_string(), 1);
        symbolic_to_octal.insert("-w-".to_string(), 2);
        symbolic_to_octal.insert("-wx".to_string(), 3);
        symbolic_to_octal.insert("r--".to_string(), 4);
        symbolic_to_octal.insert("r-x".to_string(), 5);
        symbolic_to_octal.insert("rw-".to_string(), 6);
        symbolic_to_octal.insert("rwx".to_string(), 7);

        let value = symbolic_to_octal.get(&key);
        match value{
            Some(_)=>{
                let ret_val= format!("{}",*value.unwrap());
                ret_val
            },
            None =>{
                let ret_val = format!("{:?}",self.symbolic_error_check(&key));
                ret_val
            },
        }
    }

    pub fn convert_symbolic_to_octal(&self,symbolic:String)->String{        
        if symbolic.len()!=9{
            let error_incorrect_length = "<Err: Not correct length>".to_string();
            return error_incorrect_length;
        }

        let user =&symbolic[0..3].to_string();
        let group = &symbolic[3..6].to_string();
        let owner = &symbolic[6..9].to_string();
        
        let user_octal = self.map_symbolic(user.to_string());
        let group_octal = self.map_symbolic(group.to_string());
        let owner_octal = self.map_symbolic(owner.to_string());

        let full_string = format!("{}{}{}",user_octal,group_octal,owner_octal);
        full_string
    }



    fn symbolic_error_check(&self,val:&String){
        let mut error = "".to_string();
        for character in val.chars(){
            match character{
                'r'|'w'|'x'|'-'=>{
                },
                _=>{
                    if character.is_numeric(){
                        let error_numeric = format!("<Err: {} is a number>\n",character);
                        error.push_str(&error_numeric);
                    }else if !character.is_alphabetic(){
                        let error_alphabetic = format!("<Err: {} is not alphabetic>\n",character);
                        error.push_str(&error_alphabetic);
                    }else{
                        let error_format = format!("<Err: {} is incorrect format>\n",character);
                        error.push_str(&error_format);
                    }
                }

            }
        }
        println!("{}",error);
        
    }

    fn octal_error_check(&self, val:u16){
        match val {
            8|9=>{
                let error_more_than_seven = &format!("<Err: {} is higher than 7>\n",val);
                println!("{}",error_more_than_seven);
            },
            _=>{
                
            }
            
        }
    
    
    }


}