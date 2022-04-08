for bench_workers in 1 2 3 4 5
do
	for kvs_workers in 1 2 3 4 5
	do
		echo $bench_workers $kvs_workers
		target/release/examples/kvs --read-percentage 0.5 --num-benchmark-workers $bench_workers --dist uniform,1 --run-seconds 10 --implementation noop,$kvs_workers
	done
done
