#[allow(dead_code)]
pub fn n_queens(){
    let len=4;
    let mut block=vec![vec![0;len];len];
    let mut ans=Vec::<Vec<Vec<i32>>>::new();
    fun(&mut block, 0,&mut Vec::<(usize,usize)>::new(),&mut ans);

    println!("ans.len={}",ans.len());
    for first in 0..ans.len()  {
        for x in 0..ans[first].len()  {
            for y in 0..ans[first][x].len()  {
            print!("{}\t",ans[first][x][y]);
            }
            println!("");
        }
        println!("");
    }
}
fn fun(block:&mut Vec<Vec<i32>>,lin:usize,set:&mut Vec::<(usize,usize)>,ans:&mut Vec<Vec<Vec<i32>>>){
    if lin>=block.len() {
        ans.push(block.to_vec());
        return ;
    }
    for i in 0..block[lin].len(){
        if is_to(set, (lin,i)) {
              // 复符合情况 添加
            set.push((lin,i));
            block[lin][i]=1;
            fun(block, lin+1, set,ans);
            block[lin][i]=0;
            set.pop();
        }
      
    }
}
fn is_to(set:&mut Vec::<(usize,usize)>,(x,y):(usize,usize))->bool{
    let x=x as isize;
    let y=y as isize;
    for (a,b) in set.iter() {
        let a=*a as isize;
        let b=*b as isize;
        if y==b||b-y==x-a||b-y==a-x{
            return false;
        }
    }
    true
}