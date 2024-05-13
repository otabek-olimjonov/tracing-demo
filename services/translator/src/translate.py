
def is_vowel(character):
    return character.lower() in 'aeiou'

def translate_letter(letter):
    if is_vowel(letter) or not letter.isalpha():
        return letter
    else:
        return ''.join((letter, 'o', letter.lower()))

def translate_word(word):
    return ''.join(map(translate_letter, word))
