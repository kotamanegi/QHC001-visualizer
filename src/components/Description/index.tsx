import type { FC } from 'react';

const Description: FC = () => (
  <div>
    <div>
      <a href="https://qookbook.net/coding/course/9912345/page/1/">
        問題ページに戻る
      </a>
      <br />
      <br />
      <h3>ビジュアライザの使い方</h3>
      <ul>
        <li>
          (seed=0以外で実行したいとき)
          Inputから入力をコピーし、Qookbookの入力ファイルにペースト
          または手元のファイルに移す。
        </li>
        <li>
          プログラムを実行し、標準出力に出てきた内容をコピーしてOutputにペーストする
        </li>
        <li>▶ボタンを押してアニメーションを表示する</li>
      </ul>
      このビジュアライザは
      <a href="https://yunix-kyopro.hatenablog.com/entry/2023/12/17/150534">
        yunixさんの記事
      </a>
      を参考にして作られています。
    </div>
  </div>
);

export default Description;
