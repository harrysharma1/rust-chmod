use std::collections::HashMap;

const BINDING: &str = "<Incorrect Value>";
#[allow(dead_code)]
pub struct Chmod{

}


impl Chmod{
    
    pub fn convert_octal_to_symbolic(&self,octal:u16){
        let mut octal_to_symbolic:HashMap<u16,String> = HashMap::new(); 
        octal_to_symbolic.insert(0, "---".to_string());
        octal_to_symbolic.insert(1, "--x".to_string());
        octal_to_symbolic.insert(2, "-w-".to_string());
        octal_to_symbolic.insert(3, "-wx".to_string());
        octal_to_symbolic.insert(4, "r--".to_string());
        octal_to_symbolic.insert(5, "r-x".to_string());
        octal_to_symbolic.insert(6, "rw-".to_string());
        octal_to_symbolic.insert(7, "rwx".to_string());

        if octal<100 || octal>999{
            println!("Not correct length");
            return;
        }

        let owner:u16 = octal % 10;
        let group:u16 = (octal/10)%10;
        let user:u16  = (octal/100)%10;

        let binding = BINDING.to_string();
        let user_string = octal_to_symbolic.get(&user).unwrap_or(&binding);
        let group_string = octal_to_symbolic.get(&group).unwrap_or(&binding);
        let owner_string = octal_to_symbolic.get(&owner).unwrap_or(&binding);
        print!("{}{}{}",user_string,group_string,owner_string);
        println!("");

    } 

    pub fn convert_symbolic_to_octal(&self,symbolic:String){
        let mut symbolic_to_octal: HashMap<String, u16> = HashMap::new();
        symbolic_to_octal.insert("---".to_string(), 0);
        symbolic_to_octal.insert("--x".to_string(), 1);
        symbolic_to_octal.insert("-w-".to_string(), 2);
        symbolic_to_octal.insert("-wx".to_string(), 3);
        symbolic_to_octal.insert("r--".to_string(), 4);
        symbolic_to_octal.insert("r-x".to_string(), 5);
        symbolic_to_octal.insert("rw-".to_string(), 6);
        symbolic_to_octal.insert("rwx".to_string(), 7);
        
        self.symbolic_error_check(&symbolic);

        if symbolic.len()!=9{
            println!("Not correct length");
            return;
        }

        let user =&symbolic[0..3].to_string();
        let group = &symbolic[3..6].to_string();
        let owner = &symbolic[6..9].to_string();
        
        let binding = BINDING.to_string();

        let user_octal = symbolic_to_octal.get(user);
        let group_octal = symbolic_to_octal.get(group);
        let owner_octal = symbolic_to_octal.get(owner);
        match (user_octal,group_octal,owner_octal){
            (None, None, None) => {
                print!("{}{}{}",binding,binding,binding)
            },
            (None, None, Some(_)) => {
                print!("{}{}{}",binding,binding,owner_octal.unwrap())
            },
            (None, Some(_), None) => {
                print!("{}{}{}",binding,group_octal.unwrap(),binding)
            },
            (None, Some(_), Some(_)) => {
                print!("{}{}{}",binding,group_octal.unwrap(),owner_octal.unwrap())
            },
            (Some(_), None, None) => {
                print!("{}{}{}",user_octal.unwrap(),binding,binding)
            },
            (Some(_), None, Some(_)) => {
                print!("{}{}{}",user_octal.unwrap(),binding,owner_octal.unwrap())
            },
            (Some(_), Some(_), None) => {
                print!("{}{}{}",user_octal.unwrap(),group_octal.unwrap(),binding)
            },
            (Some(_), Some(_), Some(_)) => {
                print!("{}{}{}",user_octal.unwrap(),group_octal.unwrap(),owner_octal.unwrap())
            },
        }

        println!("");
        

    }


    pub fn symbolic_error_check(&self,val:&String){
        for character in val.chars(){
            match character{
                'r'|'w'|'x'|'-'=>{

                },
                _=>{
                    if character.is_numeric(){
                        print!("-- {} is a number -- \n",character);
                    }else if !character.is_alphabetic(){
                        print!("-- {} is not even alphabetic --\n",character);
                    }else{
                        print!("-- {} is an incorrect value or in the incorrect order --\n-- Remember it has to be in the rwx format --\n",character);
                    }
                }

            }
        }
    }  
}