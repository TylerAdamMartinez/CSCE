# Tyler Adam Martinez
# Course: CSCE 3110
# Brief Description: compiles and runs the program or runs unit tests

clean:
	@echo "cleaning..."
	rm -f a.out 
	rm -f *.o 
	@echo "repo cleaned"

tests:
	@echo "======running tests======"
	g++ -Wall test.cpp lib.cpp
	./a.out

run:
	@echo "======running program======"
	g++ -Wall main.cpp lib.cpp
	./a.out
