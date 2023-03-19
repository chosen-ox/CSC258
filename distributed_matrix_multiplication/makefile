EXECS=mpi_hello_world
MPICC=/usr/lib64/mpich/bin/mpicc
MPIRUN=/usr/lib64/mpich/bin/mpirun
MPI_HOSTS=./host_file
export MPICC=/usr/lib64/mpich/bin/mpicc
export LD_LIBRARY_PATH=/usr/lib64/mpich/lib/:$LD_LIBRARY_PATH

all: build run 


build:
	cargo check

run: 
	#${MPIRUN} -n 4 -f ${MPI_HOSTS} ./${EXECS}
	cargo build  --release 
	${MPIRUN} -n 1 -f ${MPI_HOSTS}  target/release/distributed_matrix_multiplication 1 2000
	${MPIRUN} -n 2 -f ${MPI_HOSTS}  target/release/distributed_matrix_multiplication 2 2000
	${MPIRUN} -n 4 -f ${MPI_HOSTS}  target/release/distributed_matrix_multiplication 4 2000
	
clean:
	rm -f ${EXECS}
