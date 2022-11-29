# tokio
rust에서는 aysnc가 있어서 정의는 가능한데, runtime으로 따로 빠져있어서 실행은 못한다.

## tokio runtime
runtime위에 hyper, tonic이 올라간다.

# 목표
목표는 간단한 Redis구현이다.

# tokio 장점
paralell이 아닌, concurrent한 프로그램이다.
thread를 안 쓰기에 overhead가 상당히 적다.

