#include <iostream>
#include <fstream>
#include <regex>
#include <map>


using std::string;
using std::regex;

const std::map<std::string, char> NUMBER_MAP = {
    {"one", '1'},
    {"two", '2'},
    {"three", '3'},
    {"four", '4'},
    {"five", '5'},
    {"six", '6'},
    {"seven", '7'},
    {"eight", '8'},
    {"nine", '9'}
};

const string INPUT_FILE = "input.txt";

inline regex generateNumbersRegex() {
    string numbersRegex = "(?=(";
    
    for(const auto &element : NUMBER_MAP)
        numbersRegex += element.first + '|' + element.second + '|';

    numbersRegex.pop_back();
    numbersRegex += "))";

    return regex(numbersRegex);
}
const regex NUMBERS_REGEX = generateNumbersRegex();

string convertToNumbers(string input) {
    std::transform(input.begin(), input.end(), input.begin(), ::tolower);
    string output;
    std::sregex_iterator numbers(input.begin(), input.end(), NUMBERS_REGEX);
    std::sregex_iterator end;

    for(; numbers != end; numbers++) {
        if((*numbers).size() < 2) continue;

        string number = (*numbers)[1].str();

        if(number.size() == 1)
            output += number;
        else
            output += NUMBER_MAP.at(number);
    }
    
    return output;
}

int main() {

    int output = 0;
    string line;
    std::ifstream inputFile(INPUT_FILE, std::ios::in);
    if(inputFile.is_open()) {
        while(std::getline(inputFile, line)) {
            
            line = convertToNumbers(line);
            
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