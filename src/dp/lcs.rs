// 最长公共子序列
pub fn lcs(str1:&str,str2:&str)->usize{
    let mut dp=vec![vec![0;str2.len()+1];str1.len()+1];
    // let mut s1=str1.to_string().as
    for i in 1..dp.len() {
        for j in 1..dp[0].len() {
            if &str1[(i-1)..i]==&str2[(j-1)..j] {
                dp[i][j]=dp[i-1][j-1]+1;
            }else {
                dp[i][j]=std::cmp::max(dp[i-1][j],dp[i][j-1]);
            }
        }
    }
    println!("{:?}",dp);
    dp[dp.len()-1][dp[0].len()-1]
}