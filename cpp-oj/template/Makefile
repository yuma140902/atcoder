INCLUDES = -I/opt/ac-library \
					 -I/usr/include/Eigen3
debug-gcc: main.cpp
	g++-12 -std=gnu++2b -O0 -DONLINE_JUDGE -DATCODER\
		-fconstexpr-depth=2147483647 -fconstexpr-loop-limit=2147483647 -fconstexpr-ops-limit=2147483647 \
		-lgmpxx -lgmp \
		${INCLUDES} \
		$< -fanalyzer -Wall -Wextra -g -fno-omit-frame-pointer \
		-fsanitize=address -fsanitize=leak -fsanitize=undefined \
		-fsanitize=bounds -o $@

debug-clang: main.cpp
	clang++ $< -Wall -Wextra -g -O0 -fno-omit-frame-pointer \
		-fsanitize=address -fsanitize=leak -fsanitize=undefined \
		-fsanitize=bounds -o $@

optimized-gcc: main.cpp
	g++-12 -std=gnu++2b -O2 -DONLINE_JUDGE -DATCODER\
		-mtune=native -march=native \
		-fconstexpr-depth=2147483647 -fconstexpr-loop-limit=2147483647 -fconstexpr-ops-limit=2147483647 \
		-lgmpxx -lgmp \
		${INCLUDES} \
		$< -fanalyzer -Wall -Wextra -g -fno-omit-frame-pointer \
		-fsanitize=address -fsanitize=leak -fsanitize=undefined \
		-fsanitize=bounds -o $@

optimized-clang: main.cpp
	clang++ $< --analyze -Wall -Wextra -O2 -o $@

.PHONY: clean
clean:
	rm -f debug-gcc debug-clang optimized-gcc optimized-clang
