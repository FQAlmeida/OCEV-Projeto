profile = RUSTFLAGS='-g' cargo build --release; \
	valgrind --tool=callgrind --callgrind-out-file=data/profiler/callgrind.out	\
		--collect-jumps=yes --simulate-cache=yes		\
		./target/release/ga_cli -p algebraic-function -i data/instances/algebraic-function/algebraic-function.txt -c data/config/algebraic-function-test.json

profile:
	$(call profile)
