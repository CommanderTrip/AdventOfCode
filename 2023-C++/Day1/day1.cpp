#include "../solutions.h"
#include <fstream>
#include <iostream>
#include <string>
#include <cctype>

void solutionPart1() {
    // Get file and make sure it opens correctly
    std::ifstream file("C:\\Users\\jason\\Desktop\\AdventOfCode\\AoC-23-CPP\\Day1\\input.txt");
    if (!file.is_open()) {
        std::cout << "File is not open." << std::endl;
        return;
    }

    int calibrationSum = 0; // This will be the solution

    // Loop over every line and get the calibration values
    std::string line = "";
    while(std::getline(file, line)) {
        std::string calibrationValue = "AA";

        for (char i: line) {
            if (isdigit(i)) {

                // Always set the second value
                calibrationValue.at(1) = i;

                // Only set the first value once -- "A" will never be set again
                if (calibrationValue.at(0) == 'A') {
                    calibrationValue.at(0) = i;
                }
            }
        }
    
        calibrationSum += std::stoi(calibrationValue);
    }
    file.close();

    std::cout << calibrationSum;
}

void addCalibrationValue(std::string& calibrationValue, char value) {
    // Always set the second value
    calibrationValue.at(1) = value;

    // Only sent the first once
    if (calibrationValue.at(0) == 'A') calibrationValue.at(0) = value;
}

void solutionPart2() {
    // Get file and make sure it opens correctly
    std::ifstream file("C:\\Users\\jason\\Desktop\\AdventOfCode\\AoC-23-CPP\\Day1\\input.txt");
    if (!file.is_open()) {
        std::cout << "File is not open." << std::endl;
        return;
    }

    int calibrationSum = 0; // This will be the solution

    // Loop over every line and get the calibration values
    std::string line = "";
    while(std::getline(file, line)) {
        std::string calibrationValue = "AA";

        for(int i = 0; i < line.length(); i++) {
            if (isdigit(line.at(i))) {
                // Always set the second value
                calibrationValue.at(1) = line.at(i);

                // Only set the first value once -- "A" will never be set again
                if (calibrationValue.at(0) == 'A') calibrationValue.at(0) = line.at(i);
            } else {
                switch(line.at(i)) {
                    case 'o': // one
                        if (
                            i + 2 <= line.length() &&
                            line.at(i + 1) == 'n' && 
                            line.at(i + 2) == 'e'
                        ) {
                            addCalibrationValue(calibrationValue, '1');
                        }
                        break;
                    case 't': // two or three
                        if (
                            i + 2 <= line.length() &&
                            line.at(i + 1) == 'w' &&
                            line.at(i + 2) == 'o'
                        ) {
                            addCalibrationValue(calibrationValue, '2');
                        } else if (
                            i + 4 <= line.length() &&
                            line.at(i + 1) == 'h' && 
                            line.at(i + 2) == 'r' && 
                            line.at(i + 3) == 'e' &&
                            line.at(i + 4) == 'e'
                        ) {
                            addCalibrationValue(calibrationValue, '3');
                        }
                        break;
                    case 'f': // four or five
                        if (
                            i + 3 <= line.length() &&
                            line.at(i + 1) == 'o' && 
                            line.at(i + 2) == 'u' && 
                            line.at(i + 3) == 'r' 
                        ) {
                            addCalibrationValue(calibrationValue, '4');
                        } else if (
                            i + 3 <= line.length() &&
                            line.at(i + 1) == 'i' && 
                            line.at(i + 2) == 'v' && 
                            line.at(i + 3) == 'e'                           
                        ) {
                            addCalibrationValue(calibrationValue, '5');
                        }
                        break;
                    case 's': // six or seven (did it backwards...whoops)
                        if (
                            i + 4 <= line.length() &&
                            line.at(i + 1) == 'e' && 
                            line.at(i + 2) == 'v' &&
                            line.at(i + 3) == 'e' &&
                            line.at(i + 4) == 'n'  
                        ) {
                            addCalibrationValue(calibrationValue, '7');
                        } else if (
                            i + 2 <= line.length() &&
                            line.at(i + 1) == 'i' && 
                            line.at(i + 2) == 'x'                          
                        ) {
                            addCalibrationValue(calibrationValue, '6');
                        }
                        break;
                    case 'e': // eight
                        if (
                            i + 4 <= line.length() &&
                            line.at(i + 1) == 'i' && 
                            line.at(i + 2) == 'g' &&
                            line.at(i + 3) == 'h' &&
                            line.at(i + 4) == 't'  
                        ) {
                            addCalibrationValue(calibrationValue, '8');
                        }
                        break;
                    case 'n': // nine
                        if (
                            i + 3 <= line.length() &&
                            line.at(i + 1) == 'i' && 
                            line.at(i + 2) == 'n' &&
                            line.at(i + 3) == 'e' 
                        ) {
                            addCalibrationValue(calibrationValue, '9');
                        }
                        break;
                    default:
                        continue;
                }
            }
        }
    
        std::cout << line << " : " << calibrationValue << std::endl;
        calibrationSum += std::stoi(calibrationValue);
    }
    file.close();

    std::cout << calibrationSum;
}