package me.h2ofiremaster;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.LinkedList;
import java.util.List;
import java.util.Optional;
import java.util.Scanner;

public class Part2 {
    private static final File INPUT_FILE = new File("input.txt");
    
    public static void main(String[] args) {
        Scanner fileScanner;
        try {
            fileScanner = new Scanner(INPUT_FILE);
        } catch(FileNotFoundException e) {
            System.err.println("File '" + INPUT_FILE + "' was not found.");
            return;
        }
    
        LinkedList<GameRound> rounds = new LinkedList<>();
        while(fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            GameRound.fromString(line).ifPresentOrElse(
                    rounds::add,
                    () -> System.out.println("'" + line + "' failed to deserialize"));
        }
        
        int output = rounds.stream()
                .mapToInt(round -> round.getMax(CubeColor.RED)
                * round.getMax(CubeColor.GREEN)
                * round.getMax(CubeColor.BLUE))
                .sum();
        
        System.out.println("Output: " + output);
    }
    
    
    
    private enum CubeColor {
        RED, GREEN, BLUE
    }
    
    private record GameRound(int id, List<CubeStack> stacks) {
        public static Optional<GameRound> fromString(String string) {
            String[] colonSplit = string.split(":");
            if(colonSplit.length != 2) {
                System.out.println("Could not deserialize '" + string + "': More/less than one ':'");
                return Optional.empty();
            }
            String idString = colonSplit[0].substring(5);
            String[] commaSplit = colonSplit[1].split("[,;]");
            
            List<CubeStack> stacks = new LinkedList<>();
            int id;
            try {
                id = Integer.parseInt(idString);
                for(String cubeString : commaSplit) {
                    stacks.add(parseStack(cubeString));
                }
            } catch(IllegalArgumentException e) {
                System.out.println("Failed to parse string: " + string);
                return Optional.empty();
            }
            return Optional.of(new GameRound(id, stacks));
        }
        private static CubeStack parseStack(String string) throws IllegalArgumentException {
            String[] cubeStrings = string.replaceAll("^\\s*|\\s*$", "").split(" ");
            String countString = cubeStrings[0];
            String colorString = cubeStrings[1].toUpperCase();
            return new CubeStack(Integer.parseInt(countString), CubeColor.valueOf(colorString));
        }
    
        public int getMax(CubeColor color) {
            int result = 0;
            for(CubeStack stack : stacks) {
                if(stack.color != color) continue;
                result = Math.max(result, stack.count);
            }
            return result;
        }
        record CubeStack(int count, CubeColor color) {}
    }
}
