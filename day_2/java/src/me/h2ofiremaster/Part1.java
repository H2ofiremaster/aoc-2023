package me.h2ofiremaster;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Pattern;

public class Part1 {
    private static final File INPUT_FILE = new File("example.txt");
    
    private static final int SOLUTION_RED = 12;
    private static final int SOLUTION_GREEN = 13;
    private static final int SOLUTION_BLUE = 14;
    
    public static void main(String[] args) {
        Scanner fileScanner;
        try {
            fileScanner = new Scanner(INPUT_FILE);
        } catch(FileNotFoundException e) {
            System.err.println("File '" + INPUT_FILE + "' was not found.");
            return;
        }
        int output = 0;
    
        LinkedList<GameRound> rounds = new LinkedList<>();
        while(fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            rounds.add(GameRound.fromString(line));
        }
        
        rounds.stream().filter(g -> {
            return
        })
        
        System.out.println("Output: " + output);
    }
    
    
    private enum CubeColor {
        RED, GREEN, BLUE
    }
    
    private record GameRound(int id, List<CubeColor> cubeColors, List<Integer> cubeCounts) {
        private static Pattern WHITESPACE_TRIM_REGEX = Pattern.compile("");
        public static GameRound fromString(String string) {
            String[] colonSplit = string.split(":");
            if(colonSplit.length != 2) throw new IllegalArgumentException("Could not deserilize '" + string + "': More/less than one ':'");
            String idString = colonSplit[0].substring(5);
            String[] commaSplit = string.split("[,;]");
            List<CubeColor> colorList = new LinkedList<>();
            List<Integer> countList = new LinkedList<>();
            for(String cubeString : commaSplit) {
                String[] parts = cubeString.split(" ");
                
            }
        }
        private static int parseCount(String string) {
            String countString = string.replaceAll("^\\s*|\\s*$", "");
            try {
                return Integer.parseInt(countString);
            } catch(NumberFormatException e) {
            
            }
        }
        private static CubeColor parseColor(String cubeString) {
        
        }
    
        public int getMax(CubeColor color) {
            int result = 0;
            for(int i = 0; i < cubeColors.size(); i++) {
                if(cubeColors.get(i) != color) continue;
                result = Math.max(result, cubeAmounts.get(i));
            }
            return result;
        }
    
    }
}
