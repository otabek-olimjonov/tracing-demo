import unittest
import src.translate as translate

class TranslateTests(unittest.TestCase):
    def test_translate_letter(self):
        input = 'H'
        expected = 'Hoh'

        result = translate.translate_letter(input)

        self.assertEqual(result, expected)

    def test_translate_text(self):
        input = 'Hello, World!'
        expected = 'Hohelollolo, Wowororloldod!'
        result = translate.translate_text(input)

        self.assertEqual(result, expected)

    def test_empty_text(self):
        input = ''
        expected = ''
        result = translate.translate_text(input)

        self.assertEqual(result, expected)

    def test_single_vowel(self):
        input = 'a'
        expected = 'a'
        result = translate.translate_text(input)

        self.assertEqual(result, expected)

    def test_single_consonant(self):
        input = 'K'
        expected = 'Kok'
        result = translate.translate_text(input)

        self.assertEqual(result, expected)
