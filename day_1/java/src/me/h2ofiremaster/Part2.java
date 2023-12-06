package me.h2ofiremaster;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Locale;
import java.util.Map;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Part2 {
    private static final File INPUT_FILE = new File("input.txt");
    private static final Map<String, String> NUMBER_MAP = Map.of(
            "one", "1",
            "two", "2",
            "three", "3",
            "four", "4",
            "five", "5",
            "six" ,"6",
            "seven", "7",
            "eight", "8",
            "nine", "9"
    );
    private static final Pattern NUMBER_REGEX = Pattern.compile("(?=("
            + Stream.concat(NUMBER_MAP.keySet().stream(), NUMBER_MAP.values().stream())
                    .collect(Collectors.joining("|"))
            + "))");
    
    
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
            line = convertToNumbers(line);
            
            if(line.length() < 1) continue;
            String number = String.valueOf(line.charAt(0)) + line.charAt(line.length() - 1);
            
            try {
                output += Integer.parseInt(number);
            } catch(NumberFormatException e) {
                System.err.println("'" + number + "' is not a valid number.");
            }
        }
        
        System.out.println(output);
    }
    
    public static String convertToNumbers(String line) {
        String input = line.toLowerCase();
        Matcher matcher = NUMBER_REGEX.matcher(input);
        return matcher.results()
                .map(r -> r.group(1))
                .map(r -> NUMBER_MAP.getOrDefault(r, r))
                .collect(Collectors.joining(""));
    }
}
