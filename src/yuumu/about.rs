use super::prelude::*;
pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <div id="about">
                <div id="about-title">
                    <span class="title-text"> { "我是谁？" } </span>
                </div>
                <section id="about-abstract">
                    <p> { "—— 假设我们的对象语言有能力正确地谈论人类的性质，\
                    那么以下命题显然成立：" } </p>
                    <Theorem title="Theorem (Classification of Homo Sapiens)">
                        <ol><li>
                            { "存在" }
                            <Term term="典范的" trans="canonical" />
                            { "人" }
                        </li><li>
                            { "几乎所有人都是" }
                            <Term term="平凡的" trans="trivial" />
                        </li><li>
                            { "我是人的一种" }
                            <Term term="退化情况" trans="degenerate case" />
                        </li></ol>
                    </Theorem>
                </section>
                <div class="line"></div>
                <section class="about-text" id="about-1">
                    <p> { "我是谁？" } </p>
                    <p> { "显然，这其实是一个无法回答的问题。将一个人\
                    一生中扮演过的全部角色罗列出来是一件几乎不可能的事情。\
                    如果一定要给出一个答案的话……\
                    在写下这段话的时候，我决定称自己为「" }
                    <ruby>
                    { "幽夢" } <rt> { "ゆうむ" } </rt>
                    </ruby>
                    { "」。在需要同时提供姓与名的情况下，我会将其简单扩展为「" }
                    <ruby>
                    { "魂魄" } <rt> { "こんぱく" } </rt>
                    { "　" }
                    { "幽夢" } <rt> { "ゆうむ" } </rt>
                    </ruby>
                    { "」。在 ASCII 环境下，我有几个选择：除了简单的拉丁化转写，\
                    我还会使用 \"Inanimate Dream\" 作为另一种可能的选择。\
                    能否在第一时间理解这个选择的含义取决于读者对东方 Project \
                    的熟悉程度。" } </p>
                    <p> { "可能有些读者见到过别的什么，知道或者不知道\
                    那些账号在物理上属于同一个人，但几乎所有其余的都或多或少\
                    属于所谓的「历史遗留问题」。在实际生活中，我会尽量避免\
                    让不相干的事务影响无关角色的生活，但随着生活方式的转变，\
                    其中很大一部分事务甚至角色已经变得不再必要，而少数依附于\
                    其上的部分则因为系统固有的复杂性被一直依赖，导致整个\
                    角色无法被彻底废弃。由于保持一定的私密性依然是必要的，\
                    所以即使是在这里我也并不打算透露过多相关的信息。对于那些\
                    此前完全不知道「幽夢」这个称呼的朋友，我同样也希望您在\
                    读到这段文字之后可以帮助我保持这个身份不被过度打扰，\
                    本人在此先行谢过。" } </p>
                    <p> { "现在我是幽梦，可是幽梦到底是谁？" } </p>
                    <p> { "这个问题反而会容易很多：幽梦是一个沉迷于揭示抽象结构\
                    背后的关系，并喜欢用这些抽象结构作为组件搭积木玩的怪人。\
                    幽梦认为这句话已经揭示了幽梦最本质的特征。但是，\
                    显然大多数人都不会同意，并期待着一些他们想看到的「更重要」\
                    的事实。那么，基于对目标读者的偏好的观察和预测，\
                    这里额外列出了一些与幽梦有关的事实。" } </p>
                </section>
                <div class="line"></div>
                <section class="about-text" id="list-facts">
                    <ul>
                        <li> { "幽梦总是想帮人解决困难，但总是失败" } </li>
                        <li> { "幽梦很想变的漂亮，但是最终放弃了" } </li>
                        <li> { "幽梦很想和大家交朋友，但总是在害怕，不知道如何主动开口" } </li>
                        <li> { "幽梦同时喜欢幽冥组 & 千年组，但是自己却被人说像帕秋莉" } </li>
                        <li> { "幽梦讨厌市面上绝大多数编程语言" } </li>
                        <li> { "幽梦认为范畴论才是真正的宇宙通用语" } </li>
                        <li> { "幽梦喜欢玩 STG，但能通关的只有东方" } </li>
                        <li> { "幽梦想让世界变成自己所希望的样子，并且觉得大家都会喜欢" } </li>
                        <li> { "幽梦失去了几乎所有的斗争性" } </li>
                        <li> { "幽梦自己的梦想是成为一名数理逻辑学家" } </li>
                        <li> { "幽梦的性别认同是幽梦" } </li>
                    </ul>
                </section>
                <div class="line"></div>
                <section id="list-identities">
                    <a href="mailto://MLTT2HoTT@outlook.com">
                        <i class="fas fa-at fa-2x"></i>
                    </a>
                    <a href="https://github.com/YuumuKonpaku">
                        <i class="fab fa-github fa-2x"></i>
                    </a>
                    <a href="https://t.me/Cistus_Albidus">
                        <i class="fab fa-telegram fa-2x"></i>
                    </a>
                    <a href="https://twitter.com/InanimateDream_">
                        <i class="fab fa-twitter fa-2x"></i>
                    </a>
                    <a href="https://www.zhihu.com/people/wu-shi-kui-19" class="about-zhihu">
                        <div class="zhihu-box">
                            <i class="fab fa-zhihu fa-2x"></i>
                        </div>
                    </a>
                </section>
                <section id="list-friends">
                </section>
            </div>
        }
    }
}
