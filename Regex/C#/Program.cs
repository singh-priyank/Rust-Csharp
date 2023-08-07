using System;
using System.Collections.Generic;
using System.IO;
using System.Diagnostics;
using System.Text.RegularExpressions;
using Newtonsoft.Json;

namespace Regex
{
    public class Pattern
    {
        public string pattern { get; set; }
        public string label { get; set; }
    }

    public class Patterns
    {
        public List<Pattern> patterns { get; set; }
    }

    class Program
    {
        public static void RegexTest()
        {
            // Read the JSON file containing the regex patterns and replacements
            var patterns = JsonConvert.DeserializeObject<Patterns>(File.ReadAllText("patterns.json"));

            // Read the input text from the TXT file
            var inputText = File.ReadAllText("input.txt");
            string resultText = inputText;

            Dictionary<string, string> replacements = new Dictionary<string, string>();

            foreach (Pattern pattern in patterns.patterns)
            {
                replacements.Add(pattern.pattern, pattern.label);
            }

            Stopwatch stopwatch = new Stopwatch();
            stopwatch.Start();

            // Iterate over the regex patterns and replace the matches in the resultText
            foreach (var kvp in replacements)
            {
                var regex = new Regex(kvp.Key);
                resultText = regex.Replace(resultText, kvp.Value);
            }

            stopwatch.Stop();
            var duration = stopwatch.ElapsedMilliseconds;

            // Console.WriteLine("Original text:");
            // Console.WriteLine(inputText);
            // Console.WriteLine("\nReplaced text:");
            // Console.WriteLine(resultText);
            Console.WriteLine("\nTime taken: " + duration);
        }

        static void Main(string[] args)
        {
            RegexTest();
        }
    }
}