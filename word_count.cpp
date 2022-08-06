#include <iostream>
#include <sstream>
#include <string>


int main(int argc, char *argv[]) {
	
	/*
	for (int i = 1; i < argc; i++) {
		std::cout << argv[i] << std::endl;
	}
	*/
	
	bool quit = false;
	int word_count = 0;
	std::string text = "";
	std::string word = "";

	while (quit != true) {
		std::getline(std::cin, text);

		if (text == "q") {
			quit = true;
			break;
		}

		std::stringstream text_stream(text);

		while (text_stream >> word) {
			word_count++;
		}
		
		std::cout << word_count<< std::endl;
		std::cout << std::endl;

		word_count = 0;
	}
	
	return 0;
}
