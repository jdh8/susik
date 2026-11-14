# Three and four identical cards held in hand

Research checked 15 November 2026.  The question: in each game played with
the four-colour deck, what happens to three or four identical cards (same
colour, same piece) held concealed?  May a concealed triplet be split to
eat or pong a public card, or when the winning hand is laid out?  Are
generals treated differently?  Is the set declared?  Is the kong on the
fourth card compulsory, and is a dealt four locked too?  Does any game
explicitly allow the split, or lock other shapes?  The answer decides the
shape of a `Solver` option that keeps such cards together, so that
車車車馬包 reads 車車車 plus loose 馬包 and never 車車 plus 車馬包.

Claims are tagged **[text]** (a rule page or write-up says it), **[code]**
(an implementation does it; file and commit given) or **[inferred]** (my
reading, not a statement).

## Answer

**Where a source addresses the point, a concealed triplet is one unit: it
may not be split for a claim, and the only thing it may do is take its
fourth card.**  Five independent lineages say so, in three countries'
games.  Two sources go the other way, neither of them as a deliberate
rule about triplets.  Most regional games have no retrievable written
rule at all.

One fact makes the rule simpler than it looks.  In every game of this
family described below, a drawn card is turned face up and never enters
the hand, so a hand only shrinks.  Three alike in hand were therefore
always dealt; "dealt triplet" (Truong), "暗刻" and "Khạp" are the same
set, and it can be recognised from the hand alone.  **[inferred]**

| Game | Split for a claim? | Split at the win? | Generals | Declared? | Fourth card | Four dealt | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Taiwanese 十胡 (20 cards, 10 胡) | No **[text]** | No; one write-up lets the triplet's own card win as 車馬包 with the triplet intact **[text]** | Locked like any card (將將將士象 named) **[text]** | No source mentions declaring | Compulsory kong **[text]** | Stays in hand, 8 胡; splitting not addressed | Chinese Wikipedia, 瓶子; 國瑜, 大蕃薯 and both Board Game Arena pages are silent on the lock |
| Taiwanese 八胡 (18 cards) | No **[text]** | No: 馬包 must be discarded first **[text]** | Not addressed (generals may be ponged in this lineage) | Not mentioned | Compulsory **[text]** | Not addressed | 我的單車生活; i88ko and wabo copy it |
| Vietnamese Tứ sắc | No **[text]** **[code]** | No **[text]** **[code]**; whether the kong outranks a win is disputed | No exemption; Khạp of generals exists **[code]** | Quàn shown at the deal; number of Khạp announced, cards hidden **[text]** | Compulsory Khui **[text]** | Laid on the table at once **[text]** | Vinagames, binhtruong rule file, Huno705 code |
| Truong family rules (region not stated) | No **[text]** | No, by the same sentence | "Kings don't need to be in triplets"; fourth king may be stolen | Quad displayed; triplet announced with a marker, card not named **[text]** | Forced **[text]** | Displayed at once **[text]** | T. N. Truong; English Wikipedia and a Singapore dealer copy it |
| Huang family rules (region not stated) | **Yes** **[text]** | Yes, by the same sentence | Singles only | No | Optional ("can") **[text]** | Not addressed | Wei-Hwa Huang; RulesPal and English Wikipedia copy it |
| 廣水四色, Hubei (30 cards, 50 胡) | No **[text]** | No, by the same sentence | No exemption stated; 將 forms triplets at double value | Not mentioned | Compulsory (發) **[text]** | 鬧, 9 or 18 胡; splitting not addressed | 中国广水网 2011 |
| Kinmen 十胡仔 | no source found | no source found | no exemption stated | no source found | no source found | 拾八, 8 | 金門日報 names 一砍 / 開六 / 拾八 only |
| 福州四色牌 (117 cards) | no source found | no source found | Drawn 將 and 卒 may be konged, not ponged **[text]** | no source found | "可槓", not stated as compulsory | 暗槓, counted on a win | 36qp 2012, 同城游; both name 坎 without a rule |
| 豐順四色 (Hakka, Meizhou) | no source found | no source found | 暗刻 scores only for non-generals **[text]** | no source found | no source found | 暗槓 8, generals 32 | Chinese Wikipedia summary only; the operator's page is not archived |
| Vietnamese Xệp (three players) | no source found | no source found | Premium values | Sets from one's own draw go face down **[text]** | Taking is compulsory with two or three alike in hand **[text]** | Not addressed | Vinagames |
| Teochew / Shantou | no source found | | | | | | |
| Hong Kong / Cantonese | no source found | | | | | | |
| Mainland Minnan (Xiamen, Quanzhou, Zhangzhou) | no source found | | | | | | one essay, unreliable; see Caveats |
| Taiwanese Hakka (「客家二十張」) | no source found | | | | | | |
| Singapore / Malaysia | no source found | | | | | | three blog posts describe melds, none the point |
| Thailand, Cambodia | no source found | | | | | | |

### The six questions

1. **Is the concealed triplet locked?**  Yes in Taiwan (Chinese Wikipedia,
   瓶子, 我的單車生活), in Vietnam (Vinagames, the binhtruong rule file,
   Huno705's engine), in Truong's family rules and in 廣水.  The lock is
   stated first as a rule about claims — the cards may not join a public
   card in another meld — and the win follows from it: a hand that still
   holds 車車車馬包 is not finished.  No source that states the lock lets
   the triplet be split at the win.

2. **Generals.**  No source exempts them.  瓶子 names 將將將士象 beside
   車馬包包包 as the same case, and Huno705 locks a "Khạp Tướng" with the
   same function as any other.  The lock on generals is not vacuous:
   locked, 將將將士象 is three generals plus loose 士象; unlocked it is
   將將 + 將士象 and complete.  Truong's "Kings don't need to be in
   triplets" is about scoring three kings as singles, not about splitting.

3. **Declared or hidden.**  Vietnamese practice declares: a dealt four is
   laid down at once, and the *number* of Khạp is announced or marked
   with a counter while the cards stay hidden.  Truong has the same two
   rules.  No Taiwanese source mentions any declaration; 瓶子 says a
   dealt four is not even set aside.  The lock does not depend on it:
   Taiwan and 廣水 lock without declaring.  The declaration exists so
   opponents can police the forced kong and settle side payments (in
   Vietnam and Kinmen losers are paid for their own Khạp / 一砍).

4. **The fourth card.**  Compulsory wherever the lock is stated (Chinese
   Wikipedia 「必須強制開槓，漏槓包牌」, 我的單車生活 「一定要開，不然包」,
   Vinagames "must immediately meld", 廣水 「必发」, Truong "forced"),
   and in 國瑜 without the lock.  Optional only in Huang, in 福州 as worded
   ("可槓"), and in two implementations (chunhua1972 `mandatoryKong:
   false`; the Board Game Arena pages list 槓 among the actions a player
   "may" take).  A dealt four is physically removed from the hand in
   Vietnam and in Truong, so it cannot be split there.  For Taiwan and
   廣水, where it stays in hand, **no source says whether four alike may
   be read as three plus one**.

5. **Winning on the triplet's own card.**  Holding 車車車馬包 when a 車
   appears:
   - **Win as 車馬包 + 車車車**: 瓶子 (2009) **[text]**, called 殺交到, and
     only when the winning card is the triplet's own.  Vinagames ranks
     "winning priority" above "Khạp priority" and says the higher rule
     prevails **[text]**; Huno705's win check inspects only the cards
     taken from the hand, so 馬包 + 車 passes with the Khạp intact
     **[code]**, and its comment puts the win first.
   - **Must kong, then get rid of 馬包**: 我的單車生活 (2009) **[text]**,
     「必須把馬跟包打掉才能糊(因為車必須等開)」; i88ko (2024) and wabo
     copy the sentence, so this is one lineage, not three.  The binhtruong
     rule file says the same for Vietnam: "khui can not be 'dropped',
     even with win" **[text]**.
   - Chinese Wikipedia does not decide it: it has both 「自摸、胡牌者優先」
     and 「必須強制開槓」.

   In neither reading is the triplet split.  The disagreement is whether
   a win outranks the compulsory kong, which is a priority rule of the
   game and not a property of the layout.

6. **Explicit permission, and other locked shapes.**
   - Huang: a player "can break up or use melds in his hand" **[text]**.
     This is a general sentence about concealed melds in rules that have
     no concealed bonus and no forced kong; it is the only written rule
     that permits the split.  English Wikipedia carries both this
     sentence and Truong's lock, the latter as "according to some rules".
   - Truong's "when dealt, and only when dealt" reads like a limit, but
     nothing else can be a concealed triplet (see above).
   - Implementations without a lock: chunhua1972 and cedarduet, both
     built from the Board Game Arena help pages, which are silent; and
     sisepai.  What Board Game Arena itself does was not tested.
   - canhcutcon has `forbid_split_khap` as an option, off in the practice
     profile and on in a family profile that the author marks as
     unconfirmed.  It restricts *discards* only.
   - Huno705 forbids claiming or winning with part of a Khạp but lets the
     player discard from one, described as that table's variant.
   - Other shapes.  Vietnamese **bài bụng**: with 車車馬包 the pair may
     not pong a 車 (Vietnamese Wikipedia; Huno705; binhtruong "x p p m,
     can not use p p as a pair"), and Vinagames forbids any claim that
     raises the count of loose cards.  Taiwan: 「吃孤支不可拆對」 (瓶子),
     「不能單吃對子又拆手中對子」 (Chinese Wikipedia).  廣水: 「有车子必出」.
     The pairs game 九支 forbids breaking a made pair.  These are limits
     on a move given what else is in the hand, not sets that stay
     together in every layout; none needs a layout option.

## Sources

### Taiwanese 十胡 and 八胡

- **[Chinese Wikipedia 十胡][zh]** **[text]**.  Both sentences were added
  in one unsourced edit on 29 July 2018 (revision 50632556, "增加其他規則
  部分"), the same edit as the sentence on generals:

  > 手牌有暗刻（同牌同色三張）不能拆開吃、碰。
  > 當手牌暗刻的第四張牌出現時必須強制開槓，漏槓包牌。
  > 將、帥單支為一台，除了同色三隻出現第四隻需要強制開槓（開蕉）以外，
  > 一對出現第三隻不能碰。

- **[瓶子的異想世界][ccukor]**, 2009, family rules, 十胡 **[text]**:

  > 2\. 刻子不可拆：有人手上拿車車車馬包，想說車馬包當一胡，另外車車一對
  > 不算胡沒關係，但這是不行的！一定得想辦法處理掉。同理也有可能是
  > 車馬包包包或將將將士象等情形。唯一的例外是剛好胡牌的時候 … 而要胡的
  > 牌一定要是刻子的那張牌才行
  >
  > 舉例來說，手中牌有車車車馬包，來了張車即可胡牌，算車馬包( 一胡 )
  > 車車車( 三胡 ) 共四胡。若是手中牌是車車車馬，來個包則不可胡牌。

  The triplet scores 3 at that win, so it is still concealed and whole.
  A dealt four 「拿在手上，不需要像麻將特別放在旁邊，算八胡」.

- **[我的單車生活][bicycle]**, 2009, 八糊 with 18 cards **[text]**:

  > b2 開(麻將裡的槓) 一定要開，不然包。
  >
  > 手上有這個牌叫大肚，必須把馬跟包打掉才能糊(因為車必須等開)。

  The hand is a photograph I did not view; the copies below spell it out.

- **[i88ko][i88ko]** (2024) and **[wabo][wabo]** (2024), casino-affiliate
  tutorials.  Not independent: the passage splices the blog's sentence
  onto the Wikipedia one.

  > 如果手牌拿了車車車馬包，只能組成車車車／馬包，牌局上車出現就必須開
  > 不能吃，這種牌型俗稱「大肚」，必須先把馬跟包打掉才能糊（因為車必須
  > 等開）。簡單來說，手牌有暗刻（同牌同色三張）就不能拆開吃、碰。

- **Silent on the lock**: [國瑜企業社][guoyu] (card maker; has
  「玩家如果能槓牌時，一定要槓牌」), [大蕃薯][gamebase], [Board Game Arena
  English][bga] and [Chinese][bga-zh], [四色牌與麻將的關係][max],
  [wm][wm] (「有開一定要開（有槓必槓），不然會算包」).

- **Implementations.**
  - [chunhua1972/four-color-game][ch-claims] @ `ee33cf0`,
    `packages/game-core/src/rules/claims.ts` **[code]**:
    `enumerateLegalClaims` adds the offered card to the hand's counts and
    offers every pattern the counts contain, with no test for three
    alike; [`presets.ts`][ch-preset] has `mandatoryKong: false`.
  - [Zizuixixiang/cedarduet][cedar-rules] @ `31e0bfe`,
    `app/games/four_color_cards/tenhu_rules.py` `claims()` **[code]**:
    same, any pattern whose other cards are held.
  - [tw0961177543-cpu/sisepai][sise] @ `eac82fc`, `index.html`
    **[code]**: no lock; whether a held three may kong a discard is a
    house-rule switch (`kaiDiscard`, 「有些地方規定只能開自己摸的」).

### Vietnamese Tứ sắc

- **[Vinagames][tusac]**, the operator's rules, [English][tusac] and
  [Vietnamese][tusac-vi] **[text]**:

  > A quadruple set of cards originally in a hand is called a Quan. A Quan
  > must be shown to all players at the beginning of the game. A triple
  > set of cards in the private block of a hand is called a Khap. The
  > number of Khap that a player has must be announced to all players.
  >
  > Khap priority: A Khap cannot be broken, i.e. a card in a Khap cannot
  > be combined with any other private card to form a set. When the
  > active card is the same as the card in a Khap, the player owning that
  > Khap must immediately meld the active card to form a Khui and becomes
  > the current player.
  >
  > Khạp không thể bị phá. Những lá bài trong Khạp không thể dùng để chung
  > với những lá bài khác để tạo thành nhóm.

  The rules are listed "in decreasing priority", winning first, Khạp
  second.

- **[binhtruong-sj/tu-sac][binh-rules]** @ `695ee45`, `tuSac_rule.txt`,
  the author's rule notes **[text]**:

  > 3\) khui can not be 'dropped', even with win.  In the same token, khap
  > (3 cards) can not be broken.

- **[Huno705/tu-sac][huno-engine]** @ `e26b087`, `server/src/engine.ts`
  **[code]**.  `isLockedKhapCard` is true for any card held exactly three
  times, generals included.  A claim that uses such a card is refused
  unless it uses all three for the fourth (「Khạp không được phá; chỉ được
  dùng cả Khạp để Khui」), and so is a win (「Không được phá Khạp để tới」).
  `doDiscard` allows discarding from a Khạp (「Biến thể của bàn này cho
  phép phá đôi/bộ lẻ/Khạp để đánh tùy ý」).  The [README][huno-readme]:
  「Quàn phải lật lúc chia; Khạp được giữ kín với đối phương」.

- **[canhcutcon/tu_sac_terminal][canh-rules]** @ `83df231`,
  `src/tusac/rules/training.py` **[code]**.  `locked_kinds` (kinds held
  three or more times) is consulted only by `legal_discards`.
  [`rules.training.yaml`][canh-training] sets `forbid_split_khap: false`
  and calls itself 「LỰA CHỌN KỸ THUẬT cho chế độ học」; the
  [family profile][canh-family] sets it true under the question 「Có cấm
  tách khạp khi đánh?」.

- **[Vietnamese Wikipedia][vi]** **[text]**: a dealt four must be laid
  down (「cũng phải hạ Quan này xuống chiếu để mọi người cùng biết」); no
  sentence on breaking a Khạp; the section 「Bài bụng」 quoted in the
  Answer.  The many Vietnamese betting-site guides found restate
  Vinagames or Wikipedia and were not counted.

- **Huế against the south**: no source found that contrasts their Tứ sắc
  rules.  Vietnamese Wikipedia says Huế uses the deck for the
  three-player **Xệp**; [Vinagames' Xệp page][xep] **[text]** has a drawer
  who holds two or three alike take the card face down (「bắt buộc xệp」),
  lets open sets grow, and has no sentence on splitting.

### Family rules in English

- **[T. N. Truong][tnt]** **[text]**:

  > If you were dealt 4 identical cards (a quad), you must display them in
  > your playing area.
  >
  > If you were dealt 3 identical cards (a triplet), you must announce
  > that you have a triplet, without revealing which card it is.
  >
  > Also, be aware that when dealt, and only when dealt, triplets canNOT
  > be split up. For example, if you were dealt 3 White Horses, you cannot
  > make a grouping of 2 White Horses and use the other 1 White Horse to
  > make a "Field Group". This means that if the fourth card appears at
  > any time during game play, you will be forced to make a Quad grouping.

  Copied verbatim by a Singapore dealer, [Gamco][gamco]; summarised by
  [English Wikipedia][en] as "According to some rules, when a player is
  dealt a triple in their initial hand, they may not split the triple
  during play".

- **[Wei-Hwa Huang][whuang]**, archived 2008 **[text]**:

  > The player cannot use cards that are not in his hand to meld,
  > specifically, he cannot use previously opened melds (but he can break
  > up or use melds in his hand).
  >
  > If the card is needed to finish a Quad (not a Four-Footman Group),
  > the player calls "gan", and can meld the other person's card.

  Copied by [RulesPal][rulespal] and by English Wikipedia.

### Mainland games

- **[廣水四色][guangshui]**, 中国广水网, 13 March 2011, archived 2013
  **[text]**.  Quoted exactly as archived; the page's own text is
  defective in places (「车子出」 in the list of fouls must mean 不出, and
  a character may be missing before some 子).  It defines the term as
  「发子，即手中起到子（3张同色同字的牌），别人打出或从底牌被揭出第4张
  同色同字的牌，你用过来即为发子（也称“上楼”）」.

  > 有子必发，即发不能留一对，另一张再配成顺子。
  > 有车子必出，即只要有关于你车子的牌出面，就必须碰出或发子。
  > 所以“呷”、吃、碰、子不发、车子出等及牌起错（多了或少了）都属违例。
  > 违例者不能赢牌或赔胡（即倒输牌）。

  That is: a held three must take its fourth, and one may not keep a pair
  of it and fit the third card into a run.  Reading the second clause as
  the lock is mine, but I see no other sense for it.  將 is one of the 肥
  cards whose triplets score double; no exemption is stated.

- **福州四色牌**: [36qp][fuzhou] (2012, archived) and [同城游][tcy], both
  platform pages **[text]**.  「杠 明杠：手中有三张同字同色的，可杠别人
  打出的或自己摸到另一张相同的牌张」; 「摸到的将帅兵卒只能自己吃，不能被
  其他家吃、碰，可以被杠」; the 高胡 hands require 「手上没有“坎”」.
  Nothing on splitting or on compulsion.

- **豐順四色**: [Chinese Wikipedia][fengshun] only, which cites an
  operator page whose rule frame the Wayback Machine did not capture.
  It gives values (暗刻，非將 3; 將的暗槓 32) and no rule on splitting.

### Kinmen, Singapore, Malaysia

- **[金門日報][kmdn]**, 2013 **[text]**: 「手中有三張同色同字樣者（叫
  一砍）算為三。若一砍有吃到牌者叫開六 … 若手中原就有四張同樣者叫
  拾八」, and losers offset their own 一砍 against the others.  Nothing on
  splitting, compulsion or declaring.
- Singapore: [ivyidaong4][ivy] (2011), [Anthony L. esq][anthony] (2013),
  [ET Speaks From Home][ets], [Remember Singapore][remsg] (calls it "a
  favourite Teochew card game").  They describe the deck and melds; none
  addresses the point.

## Caveats

- **Lineages, not pages.**  For the lock: Chinese Wikipedia (one unsourced
  edit), 瓶子, 我的單車生活, Vinagames, Truong, 廣水, plus two Vietnamese
  programs whose authors cite Vinagames among their sources and may not
  be independent of it.  Against: Huang alone.  i88ko, wabo, Gamco,
  RulesPal, English Wikipedia and the Vietnamese betting guides are
  copies.
- **The Chinese Wikipedia sentences postdate both 2009 blogs** and have
  no citation; they may summarise the same oral rule or the blogs
  themselves.  They are testimony, not an authority.
- **Silence is not permission.**  國瑜, 大蕃薯 and both Board Game Arena
  pages say nothing either way.  The two open implementations that allow
  the split were written from those pages, so their behaviour shows what
  the pages omit, not a rule someone plays.  Board Game Arena's own
  behaviour is unknown: its source is not public and I did not play it.
- **The dealt four is the weakest point.**  Vietnamese and Truong rules
  remove it from the hand.  For Taiwan and 廣水 I found no sentence on
  whether a held four may be laid out as three plus one, or one of the
  four discarded.
- **Read verbatim** from the raw page, wikitext or source file: every
  quotation above.  Huno705's handling of 馬包 + 車 against an intact Khạp
  is my reading of the code path, not a run.
- **Not retrieved**: the 豐順 operator page (not archived); 星洲日報
  「【传统赌具／02】四色牌」 of 30 January 2023, Mobile01 thread 7004861
  and 百度百科 四色牌 (all refused the fetch); games.porg.es (404).
  Not searched: PTT and Dcard directly, video tutorials, printed sources
  such as 臺灣慣習記事.
- **Rejected**: a [美篇 essay][meipian] on "闽南四色牌" that describes
  50 胡, 发子 and 肥车 — the 廣水 vocabulary under another region's name —
  and a 比迪行銷 page that describes mahjong with replacement draws.
- **Regions with no source** (Teochew, Hong Kong, mainland Minnan,
  Taiwanese Hakka, Thailand, Cambodia) were searched in Chinese, English
  and, for Thailand, Thai.  The result is "nothing found online", not
  "no such rule".  A comment under the ivyidaong4 post says Cambodian
  rules "seem very different"; no description was found.

## Implications for susik

**One switch, off by default, and no per-rank exemption.**  The evidence
supports exactly two behaviours: the lock (Taiwan, Vietnam, Truong, 廣水)
and no lock (Huang, and what the Board Game Arena clones do).  No source
exempts generals or any other rank, and 瓶子 names generals explicitly, so
a per-rank mask would model nothing found.

**No caller-supplied locked hand.**  Every source defines the set by what
is held — three alike, concealed — and concealed cards are only ever
dealt.  Declaration changes what opponents know, never which cards are
locked.  The solver can recognise the set from the hand it is given.

**State it as "none or all".**  A meld may take a card that the hand holds
three or four times only by taking every copy.  Checked against the
sub-hand at each step this stays sound under the memo: a count of three
or four can only fall to zero, and a count of one or two never rises, so
the sub-hand alone decides and `HashMap<Hand, _>` needs no extra key.
The switch must be fixed at construction, like `values` and `loose`,
since it changes memoized answers.  With `loose` set the locked cards
never need to be loose: the triplet or quad is always available.

**Quads: lock them too, and say that this part is inferred.**  "None or
all" makes four alike a quad and nothing else.  That matches the games
that remove a dealt four from the hand, and it is the natural reading of
a rule whose point is that the set waits for, or already has, its fourth
card.  No Taiwanese or 廣水 source confirms or denies it.  The
alternative — lock at exactly three, leave four free — would let
車車車車馬包 read 車車車 + 車馬包, which is the split the rule exists to
prevent, one card later.

**Generals need no special case.**  Under "none or all" three generals in
hand are a `Triplet`, never three `Single`s and never 將將 + 將士象.  With
tsap-oo's table both readings of three generals score 3, so only the
shape reported changes.  A table that scores a general triplet unlike
three singles is the separate question in tsap-oo's
`generals-triplets-and-kongs.md`.

**The claim side needs it as much as the layout.**  Every source words
the lock as a rule about claims first.  `completions(hand, card)` should
have a locked form that drops any meld drawing on a card held three
times, except the quad of that card.  It matters for the win, not only
for eating: tsap-oo's `Round::win` tries each completion and lays out the
rest, so today 車車車馬 wins on 包 through `Field` with 車車 left as a
pair — the hand 瓶子 says cannot win.  `options_for` already filters
eats this way by hand; the win path does not.  The two belong behind one
switch so they cannot disagree, for example as a method on the solver
beside the existing free function.

**What stays in the game, not in susik.**

- *Win against kong* on the triplet's own card (question 5).  With the
  locked solver and locked completions, 車車車馬包 + 車 is a legal layout
  (`Field` from 馬包, triplet intact).  tsap-oo resolves wins before
  kongs, which is 瓶子's and Vinagames' reading; the 我的單車生活 and
  binhtruong reading is a game-side filter that refuses a win on a card
  the seat holds three of.
- *Discarding from a triplet.*  Forbidden where stated (canhcutcon's
  option, Vinagames' rule that a discard must be a loose card, the 大肚
  hand that "must wait"), allowed at Huno705's table.  That belongs with
  tsap-oo's `discardable`.
- *Compulsory kong, declaration, bài bụng and the pair rules.*  Move
  legality and table talk; none changes how a hand is laid out.

**A solver given 21 cards.**  The lock assumes the hand passed in is the
concealed hand, with the claimed card kept outside as tsap-oo does.  A
caller that merges the winning card into the hand before solving would
have 瓶子's winning layout (three 車 plus a fourth in 車馬包) rejected.
The option's documentation should say so.

[zh]: https://zh.wikipedia.org/wiki/十胡
[ccukor]: https://ccukor.pixnet.net/blog/post/37249193
[bicycle]: http://mybicyclelife.blogspot.com/2009/04/blog-post_09.html
[i88ko]: https://i88ko.com/fourcolorcardmu240106/
[wabo]: https://wabohk123.net/%E7%B7%9A%E4%B8%8A%E6%92%B2%E5%85%8B%E9%81%8A%E6%88%B2-%E5%9B%9B%E8%89%B2%E7%89%8C%E6%95%99%E5%AD%B8-%E7%8E%A9%E6%B3%95-%E6%8A%80%E5%B7%A7/
[guoyu]: https://www.guoyu.com.tw/zh/qa/1.html
[gamebase]: https://web.archive.org/web/20131005012051/http://www.gamebase.com.tw/wekey/%E5%A4%A7%E8%95%83%E8%96%AF%E9%81%8A%E6%88%B2%E7%B6%B2%E6%88%B0/%E5%9B%9B%E8%89%B2%E7%89%8C%E5%8D%81%E8%83%A1%E7%AF%87/
[bga]: https://en.doc.boardgamearena.com/Gamehelpfourcolorcards
[bga-zh]: https://zh.doc.boardgamearena.com/Gamehelpfourcolorcards
[max]: https://max670730.wordpress.com/2007/03/26/%E5%9B%9B%E8%89%B2%E7%89%8C%E8%88%87%E9%BA%BB%E5%B0%87%E7%9A%84%E9%97%9C%E4%BF%82/
[wm]: https://wmbaccrat.org/four-color-cards/
[ch-claims]: https://github.com/chunhua1972/four-color-game/blob/ee33cf080e3a3420bddc7d3cf1236d13b418dd71/packages/game-core/src/rules/claims.ts
[ch-preset]: https://github.com/chunhua1972/four-color-game/blob/ee33cf080e3a3420bddc7d3cf1236d13b418dd71/packages/game-core/src/rules/presets.ts
[cedar-rules]: https://github.com/Zizuixixiang/cedarduet/blob/31e0bfe52584cd2fbef6b8d3f0445955586675ff/app/games/four_color_cards/tenhu_rules.py
[sise]: https://github.com/tw0961177543-cpu/sisepai/blob/eac82fcb4a3aa1848c55ec7feede703652e1b3c2/index.html
[tusac]: https://www.vinagames.com/tusac.html
[tusac-vi]: https://www.vinagames.com/vi/tusac.php
[xep]: https://www.vinagames.com/xep.html
[binh-rules]: https://github.com/binhtruong-sj/tu-sac/blob/695ee45f73cbacb5e13572381767e02bf80b15a6/tuSac_rule.txt
[huno-engine]: https://github.com/Huno705/tu-sac/blob/e26b08732206d76705164510016729e4f29bbfeb/server/src/engine.ts
[huno-readme]: https://github.com/Huno705/tu-sac/blob/e26b08732206d76705164510016729e4f29bbfeb/README.md
[canh-rules]: https://github.com/canhcutcon/tu_sac_terminal/blob/83df231d02d25def7ebadbd7beb0e1637b5f09ba/src/tusac/rules/training.py
[canh-training]: https://github.com/canhcutcon/tu_sac_terminal/blob/83df231d02d25def7ebadbd7beb0e1637b5f09ba/config/rules.training.yaml
[canh-family]: https://github.com/canhcutcon/tu_sac_terminal/blob/83df231d02d25def7ebadbd7beb0e1637b5f09ba/config/rules.family_nhatrang.yaml
[vi]: https://vi.wikipedia.org/wiki/Tứ_sắc
[tnt]: https://www.cs.cmu.edu/~tnt/rules.html
[gamco]: https://gamcosales.blogspot.com/p/normal-0-false-false-false-en-sg-zh-cn_3.html
[en]: https://en.wikipedia.org/wiki/Four_color_cards
[whuang]: https://web.archive.org/web/20081210050742/http://www.ofb.net/~whuang/ugcs/gp/ssp/
[rulespal]: https://rulespal.com/see-sek/rulebook
[guangshui]: https://web.archive.org/web/20130314173229/http://jcwy.zggs.gov.cn/wtj/ViewInfo.asp?id=1618
[fuzhou]: https://web.archive.org/web/20160304084614/http://www.36qp.net/news/rule/201206/65471.shtml
[tcy]: https://www.tcy365.com/news/d24815.html
[fengshun]: https://zh.wikipedia.org/wiki/豐順四色
[kmdn]: https://www.kmdn.gov.tw/1117/1271/1274/228160
[ivy]: http://ivyidaong4.blogspot.com/2011/12/si-sek-pai-four-colour-card-game-your.html
[anthony]: https://anthonylesq.blogspot.com/2013/03/four-colour-cards-hokkien.html
[ets]: https://etspeaksfromhome.co.uk/?p=60800
[remsg]: https://remembersingapore.org/2011/11/30/4d-mahjong-and-chap-ji-kee/
[meipian]: https://www.meipian.cn/5dtbhjnt
