import java.lang.reflect.Array;
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] array= new int[]{1, 5, 2, 6, 3, 7, 4};
        int[][] commands= new int[][]{{2, 5, 3}, {4, 4, 1}, {1, 7, 3}};

        int[]reuslt= solution(array, commands);
        System.out.println(Arrays.toString(reuslt));
    }


    public static int[] solution(int[] array, int[][] commands){
        int[] answer= new int[commands.length];
        int n= 0;

        for(int i=0; i< commands.length; i++){
            int a,b,c;
            a=commands[i][0];
            b=commands[i][1];
            c=commands[i][2];

            int[] temp= Arrays.copyOfRange(array,a-1, b);

            Arrays.sort(temp);

            answer[i]= temp[c-1];
        }

        return answer;
    }
}