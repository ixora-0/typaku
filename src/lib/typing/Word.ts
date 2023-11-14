export enum CharStatus {
  Remaining,
  Correct,
  Incorrect,
  Extra
}
interface CharWithStatus {
  char: string;
  status: CharStatus;
}
export default class WordData {
  word: string;
  display: CharWithStatus[];
  next_char: string | undefined; // is undefined when there're no more chars from this.word
  next_char_idx: number;
  correctly_typed: boolean[];
  is_wrong: boolean;

  constructor(word: string) {
    this.word = word;
    this.display = word.split("").map((char) => ({ char: char, status: CharStatus.Remaining }));
    this.next_char = word[0];
    this.next_char_idx = 0;
    this.correctly_typed = new Array(this.word.length).fill(false);
    this.is_wrong = false;
  }

  is_correctly_finished(): boolean {
    return this.display.every(({ status }) => status === CharStatus.Correct);
  }

  insert_char(char: string): CharStatus {
    if (this.next_char_idx >= this.word.length) {
      this.display.push({ char, status: CharStatus.Extra });
      this.next_char_idx++;
      return CharStatus.Extra;
    }
    let status: CharStatus;
    if (char === this.next_char) {
      status = CharStatus.Correct;
      this.correctly_typed[this.next_char_idx] = true;
    } else {
      status = CharStatus.Incorrect;
    }
    this.display[this.next_char_idx] = { char, status };
    this.next_char = this.word[++this.next_char_idx];
    return status;
  }

  /**
   * @returns {boolean} - Returns `false` if already at beginning of word, `true` otherwise
   */
  backspace(): boolean {
    if (this.display.length > this.word.length) {
      this.display.pop();
      this.next_char_idx--;
      return true;
    }
    if (this.next_char_idx === 0) {
      return false;
    }
    this.next_char_idx--;
    this.next_char = this.word[this.next_char_idx];
    this.display[this.next_char_idx] = {
      char: this.next_char,
      status: CharStatus.Remaining
    };
    return true;
  }

  progress(): number {
    return this.correctly_typed.filter(Boolean).length;
  }
}
