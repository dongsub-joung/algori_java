import javax.swing.text.html.parser.Entity;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Main main= new Main();

        String[] inputList= new String[]{"119", "97674223", "1195524421"};
        var result= main.solution2(inputList);
        System.out.println(result);
    }

    public boolean solution1(String[] phoneBook) {
        Map<String, Integer> map= new HashMap<>();

        for (int i=0; i<phoneBook.length; i++)
            map.put(phoneBook[i], i);

        for(int i=0; i< phoneBook.length; i++){
            for (int j=0; j< phoneBook[i].length(); j++){
                if (map.containsKey(phoneBook[i].substring(0,j)))
                    return false;
            }
        }
        return true;
    }

    public boolean solution2(String[] phoneBook) {
        Map<String, Integer> map= new HashMap<>();

        for (int i=0; i< phoneBook.length; i++)
            map.put(phoneBook[i], i);

        for(int i=0; i< phoneBook.length; i++){
            for (int j=0; j< phoneBook[i].length(); j++){
                if (map.containsKey(phoneBook[i].substring(0,j)))
                    return false;
            }
        }

        return true;
    }
}