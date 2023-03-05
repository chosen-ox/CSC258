export OPENCILK_PATH=/home/hoover/p6/jfan30/

all: mm_cilk mm_omp mm_cuda

mm_cilk:

	${OPENCILK_PATH}/bin/clang++ -fuse-ld=lld -fopencilk -O3 -march=native -fvisibility=hidden -flto mm_cilk.cpp -o mm_cilk

mm_omp:
	c++ -fopenmp -O2 -march=native -fvisibility=hidden -flto mm_omp.c -o mm_omp

mm_cuda:
	nvcc -O3 mm_cuda.cu -o mm_cuda

clean:
	rm -rf mm_omp mm_cuda mm_cilk

test_omp: mm_omp
	./mm_omp 2000

test_cilk: mm_cilk
	CILK_NWORKERS=1 ./mm_cilk 2000
	CILK_NWORKERS=2 ./mm_cilk 2000
	CILK_NWORKERS=4 ./mm_cilk 2000
	CILK_NWORKERS=8 ./mm_cilk 2000

test_cuda: mm_cuda
	./mm_cuda 2000