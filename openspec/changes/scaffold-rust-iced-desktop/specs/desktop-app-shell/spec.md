## ADDED Requirements

### Requirement: 桌面应用可启动
系统 SHALL 提供名为 AktSQL Database Management 的 Rust 桌面应用，并使用 iced 启动。

#### Scenario: 启动应用
- **WHEN** 用户运行桌面应用
- **THEN** 打开带有 Akt 品牌的窗口

### Requirement: 工作台主壳可见
桌面应用 SHALL 显示数据库管理器工作台主壳，包含顶部栏、左侧导航、中央工作区和底部状态栏。

#### Scenario: 查看初始主壳
- **WHEN** 桌面应用打开
- **THEN** 用户看到顶部导航、侧栏导航、工作区内容和状态信息

### Requirement: 主题基础存在
桌面应用 SHALL 包含暗色和亮色主题模式，并以 Akt 红黑品牌作为默认视觉方向。

#### Scenario: 切换主题模式
- **WHEN** 用户触发主题切换
- **THEN** 应用在不关闭窗口的情况下切换暗色和亮色主题状态

### Requirement: 导航主壳有状态
桌面应用 SHALL 在应用状态中跟踪当前选中的导航区域。

#### Scenario: 选择导航项
- **WHEN** 用户选择侧栏导航项
- **THEN** 选中项被视觉标记，工作区标题随之变化

### Requirement: 验证命令已记录
仓库 SHALL 记录如何运行、格式化和检查 Rust iced 应用。

#### Scenario: 阅读仓库命令
- **WHEN** 开发者打开仓库文档
- **THEN** 能找到运行桌面应用和执行基础 Rust 验证的命令
