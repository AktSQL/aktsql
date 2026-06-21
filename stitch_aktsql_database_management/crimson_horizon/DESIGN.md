---
name: Crimson Horizon
colors:
  surface: '#121414'
  surface-dim: '#121414'
  surface-bright: '#383939'
  surface-container-lowest: '#0d0e0f'
  surface-container-low: '#1b1c1c'
  surface-container: '#1f2020'
  surface-container-high: '#292a2a'
  surface-container-highest: '#343535'
  on-surface: '#e3e2e2'
  on-surface-variant: '#e9bcb5'
  inverse-surface: '#e3e2e2'
  inverse-on-surface: '#303031'
  outline: '#b08781'
  outline-variant: '#5f3f3a'
  surface-tint: '#ffb4a8'
  primary: '#ffb4a8'
  on-primary: '#690000'
  primary-container: '#e60000'
  on-primary-container: '#fff7f5'
  inverse-primary: '#c00000'
  secondary: '#c9c6c5'
  on-secondary: '#313030'
  secondary-container: '#4a4949'
  on-secondary-container: '#bab8b7'
  tertiary: '#c8c6c5'
  on-tertiary: '#313030'
  tertiary-container: '#737272'
  on-tertiary-container: '#fbf8f7'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#ffdad4'
  primary-fixed-dim: '#ffb4a8'
  on-primary-fixed: '#410000'
  on-primary-fixed-variant: '#930100'
  secondary-fixed: '#e5e2e1'
  secondary-fixed-dim: '#c9c6c5'
  on-secondary-fixed: '#1c1b1b'
  on-secondary-fixed-variant: '#474646'
  tertiary-fixed: '#e5e2e1'
  tertiary-fixed-dim: '#c8c6c5'
  on-tertiary-fixed: '#1c1b1b'
  on-tertiary-fixed-variant: '#474746'
  background: '#121414'
  on-background: '#e3e2e2'
  surface-variant: '#343535'
typography:
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  code-md:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  label-sm:
    fontFamily: JetBrains Mono
    fontSize: 11px
    fontWeight: '500'
    lineHeight: 16px
spacing:
  unit: 4px
  container-padding: 16px
  gutter: 12px
  density-compact: 4px
  density-comfortable: 12px
---

## 品牌与风格

该设计系统服务于高性能数据库管理，灵感来自 Unix 哲学的效率与精确性，以及 Akt 鲜明的视觉识别。品牌性格应当权威、隐秘、专注工具效率。目标用户是技术型高阶用户，他们更重视思考速度和数据密度，而不是装饰性效果。

视觉风格融合 **现代极简主义** 与 **技术粗野主义**。界面使用高对比、锋利几何容器，并严格遵循功能层级。它应带来“完全掌控”的感受，也就是 `Dawn of Database Management`：复杂操作像手术一样精确、果断。

## 颜色

色板以 “Crimson Red” 为核心，只用于主操作、关键状态和品牌强调。

**暗色模式（默认）：**
主画布使用 “Deep Black” (#0A0A0A)，以降低视觉疲劳并提高对比。表面层级通过细微灰度变化 (#1A1A1A) 表达，保持扁平、技术化的观感，不依赖传统阴影。

**亮色模式：**
基础回到克制的 “Clean White” (#FFFFFF) 和 “Slate Gray” (#F5F5F5)。Crimson Red 继续作为高警示强调色，在延续品牌的同时，为日间使用提供高可读环境。

**语义颜色：**
- **成功**：Emerald Green，保留给查询完成等状态。
- **警告**：Deep Amber。
- **错误/关键**：Crimson Red。

## 字体

该设计系统采用双字体策略，用于区分 UI 导航和数据处理场景。

- **Inter（无衬线）**：用于所有结构性 UI 元素、导航和标题。它提供中性、专业的语气，让界面保持现代。
- **JetBrains Mono（等宽）**：用于所有数据密集场景，包括 SQL 编辑器、单元格值和元数据标签。

字体比例面向高密度界面。行高保持紧凑，让每英寸屏幕容纳更多信息，接近终端环境的效率。

## 布局与间距

布局在侧栏驱动导航中采用 **固定网格**，在数据表格和编辑器中采用 **流式网格**。

- **侧栏**：主导航固定为 240px。
- **主画布**：使用 12 列系统，列间距为紧凑的 12px。
- **数据密度**：间距严格基于 4px 基线网格。数据库视图中的元素使用 “Compact” 间距（4px/8px），最大化首屏可见数据。

**移动端重排：**
针对少量移动端场景，12 列网格折叠为单列，侧栏隐藏在带 Crimson 强调色的抽屉菜单后。高密度表格切换为横向滚动或卡片堆叠视图。

## 层级与深度

该设计系统拒绝柔和阴影和环境光。深度通过 **色调层级** 与 **强轮廓** 表达。

- **Level 0（基础）**：工作区背景使用 Deep Black (#0A0A0A)。
- **Level 1（面板）**：Slate Gray (#1A1A1A)，配 1px 实线边框 (#333333)。
- **Level 2（浮层/提示）**：更浅的 Gray (#2A2A2A)，顶部使用 2px Crimson Red 边框表示“激活”焦点。

交互元素不通过“浮起”表达状态，而是改变边框颜色或背景饱和度。这能保持 Unix 哲学下“扁平终端”的感觉。

## 形状

形状语言严格采用 **Sharp (0px)**。

所有 UI 元素，从按钮、输入框到大型容器卡片，都使用 90 度直角。这会形成技术精确与结构刚性的感觉。唯一突破“无圆角”规则的是 Crimson Red 云形品牌标志，它作为视觉扰动，对冲界面的冷峻锋利。

## 组件

**按钮：**
使用直角。主按钮为实心 Crimson Red，文字为 White。次级按钮为 ghost 样式，使用 1px 白色或灰色边框。不使用渐变或阴影。

**输入框：**
深色背景，仅保留 1px 底边框。聚焦时边框变为 Crimson Red。错误状态通过实心红色左边框表示。

**数据表格：**
核心组件。使用最小内边距，在 Deep Black 基础上用 #111111 做斑马纹。列头使用等宽字体与大写样式。

**标签/徽标：**
用于状态，例如 “Connected”、“Indexing”。样式为小型矩形块，使用实心色填充和等宽文字。

**导航轨：**
最左侧为 64px 细竖栏，包含锋利、极简的线性图标。激活状态通过高饱和 Crimson Red 竖线表示。

**查询编辑器：**
全宽等宽环境，语法高亮使用 Crimson 与 Slate 色板，形成定制化、品牌化的编码体验。
