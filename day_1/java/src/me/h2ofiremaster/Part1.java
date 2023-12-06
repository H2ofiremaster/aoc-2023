package me.h2ofiremaster;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class Part1 {
    private static final File INPUT_FILE = new File("input.txt");
    

    public static void main(String[] args) {
        Scanner fileScanner;
        try {
             fileScanner = new Scanner(INPUT_FILE);
        } catch(FileNotFoundException e) {
            System.err.println("File '" + INPUT_FILE + "' was not found.");
            return;
        }
        
        int output = 0;
        while(fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            
            line = line.replaceAll("[^\\d]", "");
            if(line.length() < 1) continue;
            String number = String.valueOf(line.charAt(0)) + line.charAt(line.length() - 1);
            
            try {
                output += Integer.parseInt(number);
            } catch(NumberFormatException e) {
                System.err.println("'" + number + "' is not a valid number.\n" + e);
            }
        }
    
        System.out.println(output);
    }
}
