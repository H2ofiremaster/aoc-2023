#include <iostream>
#include <fstream>
#include <regex>

using std::string;
using std::regex;

const string INPUT_FILE = "input.txt";


int main() {
    int output = 0;
    string line;
    std::ifstream inputFile(INPUT_FILE, std::ios::in);
    if(inputFile.is_open()) {
        while(std::getline(inputFile, line)) {
            line = std::regex_replace(line, regex("[^\\d]"), "");
            
            char firstChar = line[0];
            char lastChar = line[line.length() - 1];
            
            int firstInt = firstChar - '0';
            int lastInt = lastChar - '0';
            output += (firstInt * 10) + lastInt;
        }
    } else {
        std::cout << "Unable to open file: " << INPUT_FILE << std::endl;
        return -1;
    }
    
    std::cout << output << std::endl;
    return 0;
}