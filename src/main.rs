use std::io::{self, Write};

fn main() {
    std::process::Command::new("clear").status().unwrap();
    
    let mut tasks: Vec<Vec<String>> = Vec::new();
    
    tasks.push(vec!["Título |".to_string(),
                    "Descrição |".to_string(),
                    "Data de inicio |".to_string(),
                    "Data de conclusão |".to_string(),
                    "Prioridade |".to_string()]);

    loop{

        let chose: i32 = menu();
    
        if chose == 1{
            tasks = addTask(&tasks);
            std::process::Command::new("clear").status().unwrap();

            println!("Tarefa adicionada com sucesso!");
        }
        else if chose == 2{
            std::process::Command::new("clear").status().unwrap();
            
            for i in &tasks{
                println!("{:?}", i);
            }
        }
        else if chose == 3{
            tasks = editTask(tasks.clone());
            std::process::Command::new("clear").status().unwrap();

            println!("Tarefa editada com sucesso!");
        }
        else if chose == 4{
            tasks = conclusionTask(tasks.clone());
            std::process::Command::new("clear").status().unwrap();

            println!("Tarefa concluída com sucesso!");

        }
        else if chose == 5{
            tasks = removeTask(tasks.clone());
            std::process::Command::new("clear").status().unwrap();

            println!("Tarefa excluída com sucesso!");
        }
        else if chose == 6{
            return;
        }
    }

}

fn menu() -> i32{
    print!("\n");
    println!("Bem vindo ao to-do-list!");
    println!("Escolha a ação que deseja executar no menu:");
    println!("1) Adicionar tarefa"  );
    println!("2) Listar tarefas"    );
    println!("3) Editar tarefa"     );
    println!("4) Concluir tarefa"   );
    println!("5) Remover tarefa"    );
    println!("6) Sair"              );

    let mut chose: String = String::new();
    io::stdin().read_line(&mut chose);

    let chose: i32 = chose.trim().parse().unwrap();

    return chose
}

fn addTask(tasks: &Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut title = String::new();
    let mut descr = String::new();
    let mut dateI = String::new();
    let mut dateE = String::new();
    let mut prity = String::new();

    println!("Informe o título da tarefa:");
    io::stdin().read_line(&mut title);

    println!("Informe a descrição da tarefa:");
    io::stdin().read_line(&mut descr);

    println!("Informe a data de início da tarefa:");
    io::stdin().read_line(&mut dateI);

    println!("Informe a data de conclusão da tarefa:");
    io::stdin().read_line(&mut dateE);

    println!("Informe a prioridade da tarefa:");
    io::stdin().read_line(&mut prity);

    let title = title.trim().to_string();
    let descr = descr.trim().to_string();
    let dateI = dateI.trim().to_string();
    let dateE = dateE.trim().to_string();
    let prity = prity.trim().to_string();

    let mut task = tasks.to_vec();

    task.push(vec![title, descr, dateI, dateE, prity]);

    return task;
}

fn editTask(tasks: Vec<Vec<String>>) -> Vec<Vec<String>>{

    let mut title:String = String::new();
    let mut task: Vec<Vec<String>> = tasks;

    println!("Informe o título da tarefa que queira mudar");
    io::stdin().read_line(&mut title);

    let title = title.trim().to_string();

    for i in 0..task.len(){
        if task[i][0].to_string() == title{
            let mut new_title = String::new();
            let mut new_descr = String::new();
            let mut new_dateI = String::new();
            let mut new_dateE = String::new();
            let mut new_prity = String::new();
            
            println!("Informe o novo título da tarefa:");
            io::stdin().read_line(&mut new_title);

            println!("Informe a nova descrição da tarefa:");
            io::stdin().read_line(&mut new_descr);

            println!("Informe a nova data de início da tarefa:");
            io::stdin().read_line(&mut new_dateI);

            println!("Informe a nova data de conclusão da tarefa:");
            io::stdin().read_line(&mut new_dateE);

            println!("Informe a nova prioridade da tarefa:");
            io::stdin().read_line(&mut new_prity);

            let new_title = new_title.trim().to_string();
            let new_descr = new_descr.trim().to_string();
            let new_dateI = new_dateI.trim().to_string();
            let new_dateE = new_dateE.trim().to_string();
            let new_prity = new_prity.trim().to_string();

            task[i][0] = new_title;
            task[i][1] = new_descr;
            task[i][2] = new_dateI;
            task[i][3] = new_dateE;
            task[i][4] = new_prity;
            break;
        }
    }

    return task;
}

fn conclusionTask(tasks: Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut title:String = String::new();
    let mut task: Vec<Vec<String>> = tasks;

    println!("Informe o título da tarefa que queira concluir");
    io::stdin().read_line(&mut title);

    let title = title.trim().to_string();

    for i in 0..task.len(){
        if task[i][0].to_string() == title{
            task[i][4] = "concluida".to_string();
            break;
        }
    }

    return task;
}

fn removeTask(tasks: Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut title:String = String::new();
    let mut task: Vec<Vec<String>> = tasks;

    println!("Informe o título da tarefa que queira remover");
    io::stdin().read_line(&mut title);

    let title = title.trim().to_string();

    for i in 0..task.len(){
        if task[i][0].to_string() == title{
            task.remove(i);
            break;
        }
    }

    return task;
}
