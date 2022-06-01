

fn print_board(board: [[char;3];3]){

    for row in board{
        println!("{:?}", row);
    }
}


fn mark(x:usize,y:usize,player_turn: &mut String,board: &mut [[char;3];3]){
    
    if *player_turn=="Player1" {
        board[x-1][y-1] = 'X';
        *player_turn = String::from("Player2");
    } 
    else if *player_turn=="Player2" {
        board[x-1][y-1] = 'O';
        *player_turn = String::from("Player1");
    } 
    else {
        println!("{}", "Player doesnt ezist.");
    }

}


fn win_check(board: [[char;3];3]) -> bool{

    // row check 

    for row in board{
        if (row[0]==row[1] && row[1]==row[2]) && row[0]!='_' {
            if row[0]=='X'{ 
                println!("{}","Player X won");
                return true;
            } 
            else {
                println!("{}","Player Y won");
                return true;
            }
        }
    }

    // column check 

    for col in 0..board.len(){
        if (board[0][col]==board[1][col] && board[1][col]==board[2][col]) && board[0][col]!='_'{
            if board[0][col]=='X'{ 
                println!("{}","Player X won");
                return true;
            } 
            else {
                println!("{}","Player Y won");
                return true;
            }
        }
    }

    // diagonal check

    if(board[0][0]==board[1][1] && board[1][1]==board[2][2]) && board[0][0]!='_'{
        if board[0][0]=='X'{ 
            println!("{}","Player X won");
            return true;
        } 
        else {
            println!("{}","Player Y won");
            return true;
        }
    }

    if(board[0][2]==board[1][1] && board[1][1]==board[2][0]) && board[0][2]!='_'{
        if board[0][2]=='X'{ 
            println!("{}","Player X won");
            return true;
        } 
        else {
            println!("{}","Player Y won");
            return true;
        }
    }


    
    return false;

}


fn safe_position(position: &String,board: &[[char;3];3],position_x: &mut usize,position_y:&mut usize)-> bool {

    let mut position_vec: Vec<char> = Vec::new();
    position_vec.push(position.as_bytes()[0] as char);
    position_vec.push(position.as_bytes()[2] as char);
    
    *position_x = position_vec[0] as usize - '0' as usize;
    *position_y = position_vec[1] as usize - '0' as usize;

    // cleaning for invalid positions
    // println!("{}{}",*position_x,*position_y);
    if *position_x>3 || *position_x<1{
        println!("{}", "Invalid position passed for x");
        return false;
    }
    if *position_y>3 || *position_y<1{
        println!("{}", "Invalid position passed for y");
        return true
    }

    else if board[*position_x-1][*position_y-1]=='_'{
        return true;
    }

    else{
        println!("{}", "Position already occupied");
        return false;
    }

}

fn main(){

    let mut board = [['_' ; 3] ; 3];    
    let mut player_turn = String::from("Player1");
    let mut position_x: usize = 0;
    let mut position_y: usize = 0;

    
    println!("{}", "welcome to tictac toe");

    loop {
        let mut position = String::new();
        std::io::stdin().read_line(&mut position);
        println!("{}",position);


        let is_safe: bool = safe_position(&position,&board,&mut position_x,&mut position_y);
        if is_safe==false{continue;}

        
        
        mark(position_x,position_y,&mut player_turn,&mut board);
        let game_end:bool = win_check(board);
        print_board(board);   
        if game_end==true{break}

    }



}