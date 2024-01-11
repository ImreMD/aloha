use rand::{thread_rng,Rng};

struct Aloha {
    count_x1_eq1: u32,
    count_x2_eq2: u32,
    count_x1_eq2: u32,
    count_x2eq2giv_x1eq1: u32
}

fn sim_active(aloha:Aloha, p: f32, q: f32, repetition: u64) -> Aloha {
    let mut experiment = aloha;
   
    let mut rng = thread_rng();
    

    for mut n in 1..repetition {
        let mut x1 = 0;
        let mut x2 = 0;

        let mut numsend = 0;
        //let mut a_slot:i32 = 0;
        //let mut b_slot:i32 = 0;

        for mut j in 1..3 {
            let x: f32 = rng.gen_range(0.0..1.0);
            //println!("x = {}, j ={}", x ,j);
            if x < p {
                numsend = numsend + 1;
            }
            // if (j == 1) {
            //     a_slot = numsend
            // } else {
            //     b_slot = numsend
            // }
            j = j +1;
        }


        if numsend == 1 {
            x1 = 1;
        } else {
            x1 = 2;
        }


        if x1 == 2 {
            experiment.count_x1_eq2 = experiment.count_x1_eq2 + 1;
        }
        
    n = n + 1;

    //println!("| {}  | {} | {} ||", a_slot, b_slot, x1,);

    }

experiment
} 

fn main() {
    let experience = Aloha {count_x1_eq1: 0, count_x2_eq2: 0, count_x1_eq2: 0, count_x2eq2giv_x1eq1: 0};
    let result = sim_active(experience, 0.4,0.8, 2000);
    
    println!("node A success activity : {:}", result.count_x1_eq2 as f32 /2000.0);
    println!("theoretical results {}", 0.4_f32.powf(2.0) + (1.0_f32 - 0.4_f32).powf(2.0));
}
