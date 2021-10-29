package sudachi_bench;

import java.io.IOException;
import com.worksap.nlp.sudachi.Tokenizer;
import com.worksap.nlp.sudachi.Dictionary;
import com.worksap.nlp.sudachi.DictionaryFactory;
import com.worksap.nlp.sudachi.Morpheme;
import java.util.List;
import java.util.ArrayList;
import java.util.Scanner;
import java.time.Instant;
import java.time.Duration;
import java.nio.file.Paths;
import java.nio.file.Files;

public class App {
    public static void main(String[] args) throws IOException {
        String settings = Files.readString(Paths.get("sudachi.json"));
        Scanner input = new Scanner(System.in, "utf-8");
        List<String> lines = new ArrayList<String>();
        while (input.hasNext()) {
            lines.add(input.nextLine());
        }
        try (Dictionary dict = new DictionaryFactory().create(settings)) {
            Tokenizer tokenizer = dict.create();
            int n_words = 0;
            Instant start = Instant.now();
            for (String line : lines) {
                List<Morpheme> tokens = tokenizer.tokenize(Tokenizer.SplitMode.C, line);
                n_words += tokens.size();
            }
            Instant finish = Instant.now();
            double timeElapsed = (double) Duration.between(start, finish).toMillis() / 1000;
            System.out.println("Elapsed-sudachi: " + timeElapsed + " [sec]");
            System.out.println(n_words);
        }
    }
}
