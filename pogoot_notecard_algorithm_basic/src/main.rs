use rand::{prelude::thread_rng, Rng};
use text_io::read;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
fn main() {
    let result = std::fs::read_to_string("quizlet.txt").unwrap();
    let pogin = std::fs::read_to_string("quizProgress");
    let thing;
    let mut questions = if let Ok(thingy) = pogin{
        thing = thingy;
        serde_json::from_str(&thing).unwrap()
    }else{
    
    let questions = result.split("####");
    let questions = questions.map(|x|x.split("##")).map(|x|x.collect::<Vec<&str>>()).map(|x|{
        if x.len()==2{
        (x[0],x[1])
        }else{
            (x[0], "")
        }
    }).collect::<Vec<(&str, &str)>>();
    questions.into_iter().map(|x|question{turns_until_repeat:0,corrects:0,wrongs:0,front:(x.0).to_string(),back:x.1.to_string()}).filter(|x|x.front.len()>0&&x.back.len()>0).collect::<Vec<question>>()
    };

    let mut next_index =0;
    loop {
        let mut served = false;
        for i in 0..questions.len(){
            if questions[i].turns_until_repeat==0{
                //serve question
                if questions[i].corrects!=0&&questions[i].corrects>=2*questions[i].wrongs{
                    serve_question_short_answer(&mut questions[i]);
                }else{
                serve_question_multiple_choice(i, &mut questions);
                }
                served=true;
                break;
            }

        }

            if !served{
                //serve next index
                serve_question_multiple_choice(next_index, &mut questions);
                
                next_index+=1;
            }
            //lower turns for each question
            for i in 0..questions.len(){
            if questions[i].turns_until_repeat>0{
                questions[i].turns_until_repeat-=1;
            }
        }
        std::fs::write("quizProgress", serde_json::to_string(&questions).unwrap());
    }
}



fn serve_question_short_answer(mut question:&mut question){
    let mut rng = thread_rng();
    println!("{} :", question.back);
    let answer:String = read!("{}\n");
    if answer.trim()==question.front.trim(){
        println!("Correct!");
        question.corrects+=1;
        if question.corrects>question.wrongs{
            question.turns_until_repeat=rng.gen_range(20..40*question.corrects-question.wrongs);
        }else{
            question.turns_until_repeat=rng.gen_range(8..15);
        }
    }else{
        println!("Wrong!: You answered: '{}', Correct answer: '{}'",answer, question.front.trim());
        if question.wrongs>0{
        question.wrongs+=1;
        }
        question.turns_until_repeat=rng.gen_range(6..12);
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn serve_question_multiple_choice(index:usize, mut questions:&mut Vec<question>){
    if questions.len()<4{
        println!("Not enough questions");
        return;
    }
    let mut other_questions = vec![];
    for question_index in 0..questions.len(){
        if question_index!=index&&questions[question_index].back.len()>0{
            other_questions.push(&questions[question_index]);
        }
    }
    let mut question =&questions[index];
    println!("\n{}: ", question.front);
    let mut rng = thread_rng();
    let mut other_options = vec![];
    let mut count = 0;
    let mut used_indicies = vec![];
    while count<3{
        let random = rng.gen_range(0..other_questions.len());
        if !used_indicies.contains(&random){
        other_options.push(&other_questions[random ].back);
            used_indicies.push(random);
            count+=1;
        }
    }
    other_options.push(&question.back);
    other_options.shuffle(&mut rng);
    for i in 1..=other_options.len(){
        println!("\n{}. {}", i, other_options[i-1]);
    }
    println!("Input a number: ");
    let input:usize = read!();
    let input = input-1;
    if input<other_options.len()&&other_options[input]==&question.back{
        println!("Correct!");
        questions[index].corrects+=1;
        if questions[index].corrects>questions[index].wrongs{
            questions[index].turns_until_repeat=rng.gen_range(20..40*questions[index].corrects-questions[index].wrongs);
        }else{
            questions[index].turns_until_repeat=rng.gen_range(8..15);
        }
    }else{
        println!("Wrong...");
        questions[index].wrongs+=1;
        questions[index].turns_until_repeat=rng.gen_range(6..12);
    }
    std::thread::sleep(std::time::Duration::from_millis(600));
    
}

#[derive(Clone, Deserialize, Serialize)]
struct question{
    turns_until_repeat:usize,
    corrects:usize,
    wrongs:usize,
    front:String,
    back:String
}