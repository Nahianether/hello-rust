pub fn nested_array_test(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]{
    let mut result = [[0; 3]; 3];
    for i in 0..3{
        for j in 0..3{
            result[j][i] = matrix[i][j];
        }
    }
    result
}