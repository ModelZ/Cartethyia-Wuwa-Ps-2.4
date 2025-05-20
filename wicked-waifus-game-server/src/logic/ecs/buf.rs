use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicI32, Ordering};
use wicked_waifus_protocol::FightBuffInformation;

pub struct BufManager {
    active_buf_set: HashMap<i32, FightBuffInformation>,
    next_handle: AtomicI32,
    recycled_handles: HashMap<i32, VecDeque<i32>>,
}

impl BufManager {
    const PERMANENT_ROLE_BUFFS: &'static [i64] = &[

    // [(0)เปิดใช้งาน, (1)เงื่อนไข, (2)แสดงผล]
        3003,      // Remove wall run prohibition
        3004,      // Remove gliding prohibition
        1213,      // Reduce stamina while flying
        1214,      // Reduce stamina while flying in sprint
        1215,      // Reduce stamina while flying up in sprint
        1216,      // Reduce stamina while flying down in sprint
        640012051, // Allow flying -> tag: 1151923109
 
        1607101000, // Forte Buff (Cantarella)
        1607101331, // E Forte (Cantarella)
        1607301307,
        1607301308,
        1607700006,
        1406010006,
        1607220000,
        1607220001,
        1607510002,
        1607520002,
        1507002003,//zani
        1507501015,
        150721031,
        150721030,
        1507002007,
        1507701004,
        1507701015,
        1507002011,
        150723111,
        1507002013,
        1507501007,
        1507002001,
        1507701009,
        9100000020001,//The Crownless
        938000401021,
        938000402040,
        938000402060,
        938000401022,
        938000406002,
        1205104001,//Changli
        1205104044,
        1205104002,
        1205104023,
        1205104024,
        1205104041,
        1407900004,//Ciaconna
        1407200300,
        1407200200,
        1407900002,
        1407000012,
        1407000013,
        1407000014,
        1407000015,

 //cartethyia
                 1409200114, // (0), 加2特殊能量-64号属性-风剑强化 // Translated: Add 2 special energy - No. 64 attribute - Wind Sword Enhancement
                 1409200127, // (0), 加5特殊能量-64号属性-水剑强化 // Translated: Add 5 special energy - No. 64 attribute - Water sword enhancement
                 1409200003, // (0), 清空特殊能量-62号属性 // Translated: Clear special energy-attribute No. 62
                 1409200115, // (0), 加3特殊能量-64号属性-风剑强化 // Translated: Add 3 special energy - No. 64 attribute - Wind Sword Enhancement
                 1409200133, // (0), 加1特殊能量-64号属性-重力剑强化 // Translated: Add 1 special energy - No. 64 attribute - Gravity sword enhancement
                 1409200105, // (0), 加3特殊能量-64号属性 // Translated: Add 3 special energy - No. 64 attribute
                 1409200100, // (0), 加10特殊能量-64号属性 // Translated: Add 10 special energy-No. 64 attributes
                 1409200146, // (0), 加4特殊能量-64号属性 // Translated: Add 4 special energy - No. 64 attribute
                 1409200002, // (0), 减1特殊能量-62号属性 // Translated: Reduce 1 special energy-attribute No. 62
                 1409200126, // (0), 加4特殊能量-64号属性-水剑强化 // Translated: Add 4 special energy - No. 64 attribute - Water sword enhancement
                 1409200123, // (0), 加1特殊能量-64号属性-水剑强化 // Translated: Add 1 special energy - No. 64 attribute - Water sword enhancement
                 1409200104, // (0), 加2特殊能量-64号属性 // Translated: Add 2 special energy - No. 64 attribute
                 1409200102, // (0), 减30特殊能量-64号属性 // Translated: Reduce 30 special energy - No. 64 attribute
                 1409401081, // (0), 入场技协奏能量-小 // Translated: Entrance technical concerto energy-small
                 1409303002, // (0), 风-1 // Translated: Wind-1
                 1409401084, // (0), 普通E协奏能量 // Translated: Ordinary E Concerto Energy
                 1409200113, // (0), 加1特殊能量-64号属性-风剑强化 // Translated: Add 1 special energy - No. 64 attribute - Wind sword enhancement
                 1409200136, // (0), 加4特殊能量-64号属性-重力剑强化 // Translated: Add 4 special energy - No. 64 attribute - Gravity sword enhancement
                 1409200124, // (0), 加2特殊能量-64号属性-水剑强化 // Translated: Add 2 special energy - No. 64 attribute - Water sword enhancement
                 1409200116, // (0), 加4特殊能量-64号属性-风剑强化 // Translated: Add 4 special energy - No. 64 attribute - Wind Sword Enhancement
                 1409200103, // (0), 加1特殊能量-64号属性 // Translated: Add 1 special energy-No. 64 attribute
                 1409200107, // (0), 加5特殊能量-64号属性 // Translated: Add 5 special energy - No. 64 attribute
                 1409200137, // (0), 加5特殊能量-64号属性-重力剑强化 // Translated: Add 5 special energy - No. 64 attribute - Gravity sword enhancement
                 1409200117, // (0), 加5特殊能量-64号属性-风剑强化 // Translated: Add 5 special energy - No. 64 attribute - Wind sword enhancement
                 1409200101, // (0), 减10特殊能量-64号属性 // Translated: Reduce 10 special energy - No. 64 attribute
                 1409200106, // (0), 清空大招能量 // Translated: Clear the ultimate power
                 1409200001, // (0), 加1特殊能量-62号属性 // Translated: Add 1 special energy-No. 62 attribute
                 1409401010, // (0), 开大清除爆伤buff // Translated: Open the big and clear the buff
                 1409401086, // (0), 技能1强化版协奏能量-大 // Translated: Skill 1 Enhanced Edition Concerto Energy-Big
                 1409303001, // (0), 立即结算一次异常 // Translated: Check out an exception immediately
                 1409200134, // (0), 加2特殊能量-64号属性-重力剑强化 // Translated: Add 2 special energy - No. 64 attribute - Gravity sword enhancement
                  1409303003, // (0), 大招根据敌人风层数转换加深，每层20% // Translated: The ultimate move deepens according to the enemy's wind layer number, with 20 per cent per layer
                  1409406002, // (0), 大招根据敌人风层数转换加深，每层20% // Translated: The ultimate move deepens according to the enemy's wind layer number, with 20 per cent per layer
                  1409406022, // (0), 芙露德莉斯-共鸣链6——上99层风异常 // Translated: Fuludris-Resonance Chain 6--The upper 99th layer of wind abnormality
                  1409301001, // (0), 开大扣除至血量上限的30% // Translated: Open a large deduction to the upper limit of blood volume
                  1409200135, // (0), 加3特殊能量-64号属性-重力剑强化 // Translated: Add 3 special energy - No. 64 attribute - Gravity sword enhancement
                  1409200125, // (0), 加3特殊能量-64号属性-水剑强化 // Translated: Add 3 special energy - No. 64 attribute - Water sword enhancement
                  1409301103, // (0), 初始化时，若没有水tag，则-1特殊能量 // Translated: During initialization, if there is no water tag, then -1 special energy
                  1409301102, // (0), 初始化时，若没有重力tag，则-1特殊能量 // Translated: During initialization, if there is no gravity tag, then -1 special energy
                  1409401085, // (0), 技能1协奏能量-大 // Translated: Skill 1 Concerto Energy-Big
                  1409402015, // (0), 下落攻击刷新E技能CD（2档） // Translated: Drop attack refreshes E skill CD (2nd level)
                  1409402016, // (0), 下落攻击刷新E技能CD（3档） // Translated: Drop attack refreshes E skill CD (3 levels)
                  1409402014, // (0), 下落攻击刷新E技能CD（1档） // Translated: Refresh E skill CD (1st level)
                  1409401082, // (0), E协奏能量-小 // Translated: E Concerto Energy-Small
                  1409406013, // (0), 立即结算一次异常 // Translated: Check out an exception immediately
                  1409402013, // (0), 立即结算一次异常 // Translated: Check out an exception immediately
                  1409401083, // (0), R协奏能量-小 // Translated: R Concerto Energy-Small
                  1409301101, // (0), 初始化时，若没有风tag，则-1特殊能量 // Translated: During initialization, if there is no wind tag, then -1 special energy
                  1409301002, // (0), 二段大回复当前血量上限值的70% // Translated: Second stage large reply 70% of the current upper limit of health

         //1409001022, // (1), 小形态--幻影剑2存在 // Translated: Small form-Phantom Sword 2 exists
         //1409200204, // (1), 手环lv1材质 // Translated: Bracelet lv1 material
         //1409200201, // (1), 手环材质监控-lv1 // Translated: Bracelet material monitoring-lv1
           //1409001003, // (1), 大形态标记 // Translated: Large shape mark
           //1409001039, // (1), 大神形态-重力幻影剑技能按钮用tag // Translated: Great God Form - Gravity Phantom Sword Skill Tag
           //1409001005, // (1), 圣剑能量监听-满120给tag // Translated: Holy Sword Energy Monitor-All 120 for tags
           //1409200203, // (1), 手环材质监控-lv3 // Translated: Bracelet material monitoring-lv3
           //1409001093, // (1), 小形态标记 // Translated: Small form mark
           //1409204011, // (1), 幻影剑1附着 // Translated: Phantom Sword 1 attached
           //1409001006, // (1), 满能量tagbuff // Translated: Full energy tagbuff
           //1409001001, // (1), 大形态基础镜头 // Translated: Large-form basic lens
           //1409001013, // (1), 小变大变装功能 // Translated: Small to large drag function
           //1409200206, // (1), 手环lv3材质 // Translated: Bracelet lv3 material
           //1409001033, // (1), 小形态--幻影剑3能量-水tag // Translated: Small form-Phantom Sword 3 Energy-water tag
           //1409200205, // (1), 手环lv2材质 // Translated: Bracelet lv2 material
           //1409200202, // (1), 手环材质监控-lv2 // Translated: Bracelet material monitoring-lv2
           //1409302023, // (1), 小神卡技能—风+3 // Translated: Little God Card Skill—Wind 3
           //1409001021, // (1), 小形态--幻影剑1存在 // Translated: Small form-Phantom Sword 1 exists
           //1409401063, // (1), E触发器-小 // Translated: E Trigger - Small
           //1409401069, // (1), 下落攻击触发器-大 // Translated: Whereabouts Attack Trigger - Large
           //1409001023, // (1), 小形态--幻影剑3存在 // Translated: Small form--Phantom Sword 3 exists
           //1409001002, // (1), 大形态战斗镜头 // Translated: Large form of combat shot
           //1409201003, // (1), 大形态幻影剑3显示-水 // Translated: Large form Phantom Sword 3 display-water
           //1409701001, // (1), 挂载被动技能清单-katixiya // Translated: Mount passive skills list - katixiya
           //1409001031, // (1), 小形态--幻影剑1能量-重力tag // Translated: Small form-Phantom Sword 1 Energy-Gratility Tag
           //1409205001, // (1), 空中攻击次数 // Translated: Number of air attacks
           //1409401999, // (1), 触发器 // Translated: trigger
           //1409204012, // (1), 幻影剑2附着 // Translated: Phantom Sword 2 attached
           //1409001032, // (1), 小形态--幻影剑2能量-风tag // Translated: Small form-Phantom Sword 2 Energy-Wind tag
           //1409401051, // (1), 加tag1 // Translated: Add tag1
           //1409401070, // (1), R-触发器-大 // Translated: R-trigger-large
           //1409401068, // (1), E触发器-大 // Translated: E Trigger - Large
           //1409406001, // (1), 大招触发器 // Translated: The ultimate trigger
           //1409302021, // (1), 小神卡普攻最后一段—风+1 // Translated: The last part of the little god Cap attack - Wind 1
           //1409401053, // (1), 加tag3 // Translated: Add tag3
           //1409401067, // (1), 重击&QTE剑触发器-大 // Translated: Hit
           //1409200207, // (1), 手环lv3材质（全吸收时） // Translated: Bracelet lv3 material (when fully absorbed)
           //1409403003, // (1), 强化重击-风+2 // Translated: Strengthening heavy blow-Wind 2
           //1409403002, // (1), 强化E-风+2 // Translated: Strengthen E-wind 2
           //1409001007, // (1), 小卡+凭依TAG(技能图标用) // Translated: Small card, rely on TAG (for skill icon)
           //1409401062, // (1), 重击&QTE剑触发器-小 // Translated: Hit
           //1409403001, // (1), 空中攻击第1段-风+2 // Translated: Air Attack Paragraph 1 - Wind 2
           //1409401054, // (1), 加tag4 // Translated: Add tag4
           //1409401052, // (1), 加tag2 // Translated: Add tag2
           //1409402001, // (1), 风异常上限加3光环 // Translated: Wind abnormality upper limit plus 3 halos
           //1409001042, // (1), 小形态--幻影剑2能量-风神符特效（暂时废弃隐藏） // Translated: Small form-Phantom Sword 2 Energy-Wind God Talisman Special Effect (temporarily abandoned and hidden)
           //1409401006, // (1), 90能量 // Translated: 90 Energy
           //1409201002, // (1), 大形态幻影剑2显示-风（同时附带部分效果） // Translated: Large form Phantom Sword 2 display-wind (with some effects at the same time)
           //1409401097, // (1), 风神退场【被动2】-【被动2tag，解锁被动2挂这个buff】 // Translated: Feng Shen exits [Passive 2]-[Passive 2 tag, unlock passive 2 hanging this buff]
            //1409401092, // (1), 风效应>4层时，大神卡伤害提升30% // Translated: When the wind effect > 4th floor, the damage of the master card is increased by 30 %
            //1409406023, // (1), 芙露德莉斯大招使目标受到的伤害提升40% // Translated: Furudlis' ultimate move increases the damage taken by the target by 40 %
            //1409406021, // (1), 芙露德莉斯使目标受到的伤害提升40% // Translated: Fludris increases the damage taken by the target by 40 %
            //1409302301, // (1), 被动1：风效应3层时，怪物受到的伤害增加30% // Translated: Passive 1: When the wind effect is 3, the damage taken by monsters increases by 30 %
            //1409001043, // (1), 小形态--幻影剑3能量-水神符特效（暂时废弃隐藏） // Translated: Small form-Phantom Sword 3 Energy-Water God Talisman Special Effect (temporarily abandoned and hidden)
            //1409204013, // (1), 幻影剑3附着 // Translated: Phantom Sword 3 attached
            //1409302022, // (1), 小神卡重击—风+2 // Translated: Little God Card Strike - Wind 2
            //1409001041, // (1), 小形态--幻影剑1能量-重力神符特效（暂时废弃隐藏） // Translated: Small form-Phantom Sword 1 Energy-Grave God Talisman Special Effect (temporarily abandoned and hidden)
            //1409201001, // (1), 大形态幻影剑1显示-重力（同时附带部分效果） // Translated: Large form Phantom Sword 1 display - gravity (also included with some effects)
            //1409302601, // (1), 被动1：风效应6层时，怪物受到的伤害增加60% // Translated: Passive 1: When the Wind Effect is 6th level, the damage taken by the monster increases by 60 %
            //1409302013, // (1), 风效应>6层时，小神卡重击增伤60%（废弃） // Translated: When the wind effect > 6th floor, the little god card will hit hard and increase damage by 60 % (discarded)
            //1409302501, // (1), 被动1：风效应5层时，怪物受到的伤害增加50% // Translated: Passive 1: When the wind effect is 5th level, the damage taken by the monster increases by 50 %
            //1409001044, // (1), 小形态--幻影剑能量-底座（暂时废弃隐藏） // Translated: Small form-Phantom Sword Energy-Pole (temporarily abandoned and hidden)
            //1409302401, // (1), 被动1：风效应4层时，怪物受到的伤害增加40% // Translated: Passive 1: When the Wind Effect is 4th level, the damage taken by the monster increases by 40 %
            //1409401003, // (1), 击杀目标时记录层数并在下一个技能命中 // Translated: Record the number of layers when killing the target and hit the next skill
            //1409401066, // (1), 普攻剑触发器-大 // Translated: Normal attack sword trigger-big
            //1409404002, // (1), 致命伤害时触发buff // Translated: Trigger buff when fatal damage
            //1409401004, // (1), 30能量 // Translated: 30 energy
            //1409401055, // (1), 加tag5 // Translated: Add tag5
            //1409401064, // (1), 下落攻击触发器-小 // Translated: Whereabouts Attack Trigger - Small
            //1409401056, // (1), 加tag6 // Translated: Add tag6
            //1409403004, // (1), 普攻第3段-风+2 // Translated: General Attack Section 3 - Wind 2
            //1409401007, // (1), 120能量 // Translated: 120 energy
            //1409402011, // (1), 标识传递 // Translated: Identity delivery
            //1409401065, // (1), R-触发器-小 // Translated: R-trigger-small
            //1409401040, // (1), 触发器-开大后风伤加成 // Translated: Trigger - Wind injury bonus after opening
            //1409401093, // (1), 风神退场被动0【触发器】 // Translated: Feng Shen exit passive 0 [trigger]
            //1409401001, // (1), 添加buff // Translated: Add buff
            //1409302024, // (1), 小神卡QTE—风+2 // Translated: Xiao Shen Card QTE—Wind 2
            //1409404001, // (1), 免死 // Translated: Avoid death
            //1409001008, // (1), 大卡+凭依TAG(技能图标用) // Translated: Big card, rely on TAG (for skill icon)
            //1409401042, // (1), 芙露德莉斯大招风伤加成60% // Translated: Fludelis's ultimate attack on wind injury bonus of 60 %
            //1409401005, // (1), 60能量 // Translated: 60 Energy
            //1409201011, // (1), 大形态幻影剑1效果1-重力空中抗打断 // Translated: Large-form Phantom Sword 1 Effect 1 - Gravity Air Resistance to Break
            //1409401041, // (1), 风伤加成60% // Translated: Wind injury bonus 60 %
            //1409401009, // (1), 监听开大 // Translated: Listen to the big
            //1409401061, // (1), 普攻剑触发器-小 // Translated: Normal attack sword trigger-small
            //1409405001, // (1), 异常触发器 // Translated: Exception trigger
            //1409302201, // (1), 被动1：风效应2层时，怪物受到的伤害增加20% // Translated: Passive 1: When Wind Effect Level 2, the damage taken by monsters increases by 20 %
            //1409401091, // (1), 风效应>4层时，小神卡伤害提升30% // Translated: When the wind effect > 4th floor, the damage of the little god card is increased by 30 %
            //1409302101, // (1), 被动1：风效应1层时，怪物受到的伤害增加10% // Translated: Passive 1: When the wind effect level 1, the damage taken by the monster increases by 10 %


//Lupa       
         //1207, // (0), 进入快速攀爬体力扣减通用 // Translated: Enter the fast climbing and physical strength deduction for general
         //12070000310, // (0), [基础特殊能量]下落攻击2 // Translated: [Basic Special Energy] Drop Attack 2
         //12070000334, // (0), [基础特殊能量]QTE-2 4hit // Translated: [Basic Special Energy] QTE-2 4hit
         //12070000100, // (0), 露帕特殊能量加5-基础 // Translated: Lupa special energy plus 5-basic
         //12070000331, // (0), [基础特殊能量]终极E2-4hit // Translated: [Basic Special Energy] Ultimate E2-4hit
           //12070000341, // (0), [基础特殊能量]冲刺技能2 // Translated: [Basic Special Energy] Sprint Skill 2
           //12070000307, // (0), [基础特殊能量]普攻4-2 // Translated: [Basic Special Energy] General Attack 4-2
           //12070000339, // (0), [基础特殊能量]下落派生2 // Translated: [Basic Special Energy] Whereabouts 2
           //12070100302, // (0), 移除-强化E2-可释放 // Translated: Remove-E2-releaseable
           //12070000401, // (0), [开大特殊能量]普攻1-2 // Translated: [Open the special energy] General attack 1-2
           //12070000326, // (0), [基础特殊能量]基础技能1-伤害 // Translated: [Basic Special Energy] Basic Skill 1-Damage
           //12070000101, // (0), 露帕特殊能量加10-基础 // Translated: Lupa special energy plus 10-basic
           //12070000110, // (0), 露帕特殊能量加25-强化 // Translated: Lupa special energy plus 25-enhanced
           //12070000303, // (0), [基础特殊能量]普攻2-1 // Translated: [Basic Special Energy] General Attack 2-1
           //12070000301, // (0), [基础特殊能量]普攻1-2 // Translated: [Basic Special Energy] General Attack 1-2
           //12070000302, // (0), [基础特殊能量]普攻1-3 // Translated: [Basic Special Energy] General Attack 1-3
           //12070000317, // (0), [基础特殊能量]空中普攻3-2 // Translated: [Basic Special Energy] Air normal attack 3-2
           //12070000300, // (0), [基础特殊能量]普攻1-1 // Translated: [Basic Special Energy] General Attack 1-1
           //12070000305, // (0), [基础特殊能量]普攻3-2 6hit // Translated: [Basic Special Energy] General Attack 3-2 6hit
           //12070000103, // (0), 露帕特殊能量加50-基础 // Translated: Lupa special energy plus 50-basic
           //12070000314, // (0), [基础特殊能量]空中普攻2-1 // Translated: [Basic Special Energy] Air normal attack 2-1
           //12070000304, // (0), [基础特殊能量]普攻3-1 // Translated: [Basic Special Energy] General Attack 3-1
           //12070000313, // (0), [基础特殊能量]空中普攻1-挑飞 // Translated: [Basic Special Energy] Air General Attack 1-Choose
           //12070910101, // (0), 露帕隐藏常态尾巴 // Translated: Lupa hides normal tail
           //12070600103, // (0), 刷新E技能CD // Translated: Refresh E-Skill CD
           //12070000330, // (0), [基础特殊能量]终极E1 // Translated: [Basic Special Energy] Ultimate E1
           //12070000337, // (0), [基础特殊能量]终结E连携技普攻 // Translated: [Basic Special Energy] End E-connected technology general attack
           //12070000328, // (0), [基础特殊能量]强化技能1-飞枪 // Translated: [Basic Special Energy] Strengthening Skill 1-Flying Gun
           //12070000315, // (0), [基础特殊能量]空中普攻2-2 4hit // Translated: [Basic Special Energy] Air General Attack 2-2 4hit
           //12070000329, // (0), [基础特殊能量]大招伤害1 // Translated: [Basic Special Energy] Damage 1 of the ultimate move
           //12070000319, // (0), [基础特殊能量]基础重击1-2 // Translated: [Basic Special Energy] Basic heavy blow 1-2
           //12070000320, // (0), [基础特殊能量]强化重击1-1 // Translated: [Basic Special Energy] Enhanced heavy blow 1-1
           //12070000111, // (0), 露帕特殊能量加50-强化 // Translated: Lupa special energy plus 50-enhanced
           //12070000400, // (0), [开大特殊能量]普攻1-1 // Translated: [Open the special energy] General attack 1-1
           //12070000340, // (0), [基础特殊能量]冲刺技能1 // Translated: [Basic Special Energy] Sprint Skill 1
           //12070000112, // (0), 露帕特殊能量加75-强化 // Translated: Lupa special energy plus 75-enhanced
           //12070000318, // (0), [基础特殊能量]基础重击1-1 // Translated: [Basic Special Energy] Basic heavy blow 1-1
           //12070000338, // (0), [基础特殊能量]下落派生1 // Translated: [Basic Special Energy] Whereabouts 1
           //12070910201, // (0), 露帕隐藏火焰尾巴 // Translated: Lupa hides flame tail
           //12070910102, // (0), 露帕显示常态尾巴 // Translated: Lupa shows normal tail
           //12070000311, // (0), [基础特殊能量]闪避反击1 4hit // Translated: [Basic Special Energy] Dodge Counterattack 1 4hit
           //12070000424, // (0), [开大特殊能量]强化重击2-3 // Translated: [Open the special energy] Enhance the heavy blow 2-3
           //12070000415, // (0), [开大特殊能量]空中普攻2-2 4hit // Translated: [Open the special energy] Air normal attack 2-2 4ht
           //12070000333, // (0), [基础特殊能量]QTE-1 // Translated: [Basic Special Energy] QTE-1
           //12070000332, // (0), [基础特殊能量]终极E3 // Translated: [Basic Special Energy] Ultimate E3
           //12070000407, // (0), [开大特殊能量]普攻4-2 // Translated: [Open the special energy] General attack 4-2
           //12070000410, // (0), [开大特殊能量]下落攻击2 // Translated: [Open the special energy] Fall attack 2
           //12070000201, // (0), 露帕特殊能量减50-基础 // Translated: Lupa special energy reduction 50-basic
           //12070000405, // (0), [开大特殊能量]普攻3-2 6hit // Translated: [Open the special energy] General attack 3-2 6hit
           //12070000412, // (0), [开大特殊能量]闪避反击2 // Translated: [Open the special energy] Dodge Counterattack 2
           //12070000406, // (0), [开大特殊能量]普攻4-1 // Translated: [Open the special energy] General attack 4-1
           //12070000428, // (0), [开大特殊能量]强化技能1-飞枪 // Translated: [Open the special energy] Enhanced skill 1-Flying gun
           //12070000427, // (0), [开大特殊能量]基础技能2-插枪 // Translated: [Open the special energy] Basic skill 2-Insert the gun
           //12070000322, // (0), [基础特殊能量]强化重击2-1 // Translated: [Basic Special Energy] Enhanced heavy blow 2-1
           //12070000439, // (0), [开大特殊能量]下落派生2 // Translated: [Open the special energy] Whereabouts 2
           //12070000440, // (0), [开大特殊能量]冲刺技能1 // Translated: [Open the special energy] Sprint skill 1
           //12076120004, // (0), 露帕终结E连携技切人不隐藏移除 // Translated: Lupa End E-connection technology cuts people without hiding and removing
           //12070000321, // (0), [基础特殊能量]强化重击1-2 // Translated: [Basic Special Energy] Enhanced heavy blow 1-2
           //12070000430, // (0), [开大特殊能量]终极E1 // Translated: [Open the special energy] Ultimate E1
           //12070000309, // (0), [基础特殊能量]下落攻击1 // Translated: [Basic Special Energy] Drop Attack 1
             //12070000438, // (0), [开大特殊能量]下落派生1 // Translated: [Open the special energy] Whereabouts 1
             //12079010102, // (0), 露帕大招光环（光环移除） // Translated: Lupa ultimate aura (halo removal)
             //12070000423, // (0), [开大特殊能量]强化重击2-2-4hit // Translated: [Open the special energy] Enhanced heavy blow 2-2-4hit
             //12070000104, // (0), 露帕特殊能量加100-基础 // Translated: Lupa special energy plus 100-Basic
             //12070000308, // (0), [基础特殊能量]普攻4-3 // Translated: [Basic Special Energy] General Attack 4-3
             //12070000336, // (0), [基础特殊能量]终结E连携技聚怪 // Translated: [Basic Special Energy] End E-connection skill gathers monsters
             //12070000420, // (0), [开大特殊能量]强化重击1-1 // Translated: [Open the special energy] Enhanced heavy blow 1-1
             //12070000422, // (0), [开大特殊能量]强化重击2-1 // Translated: [Open the special energy] Enhance the heavy blow 2-1
             //12070000327, // (0), [基础特殊能量]基础技能2-插枪 // Translated: [Basic Special Energy] Basic Skill 2-Insert Gun
             //12070000202, // (0), 露帕特殊能量减100-基础 // Translated: Lupa special energy reduction 100-basic
             //12070000432, // (0), [开大特殊能量]终极E3 // Translated: [Open the special energy] Ultimate E3
             //12070000418, // (0), [开大特殊能量]基础重击1-1 // Translated: [Open the special energy] Basic heavy blow 1-1
             //12070000316, // (0), [基础特殊能量]空中普攻3-1 // Translated: [Basic Special Energy] Air normal attack 3-1
             //12070000429, // (0), [开大特殊能量]大招伤害1 // Translated: [Open the special energy] The ultimate move damage 1
             //12070000421, // (0), [开大特殊能量]强化重击1-2 // Translated: [Open the special energy] Enhance the heavy blow 1-2
             //12070000434, // (0), [开大特殊能量]QTE-2 4hit // Translated: [Open the special energy] QTE-2 4hit
             //12070000435, // (0), [开大特殊能量]终结E连携技伤害 // Translated: [Open the special energy] End the damage of E-connected technology
             //12070000433, // (0), [开大特殊能量]QTE-1 // Translated: [Open the special energy] QTE-1
             //12070000436, // (0), [开大特殊能量]终结E连携技聚怪 // Translated: [Open the special energy] End the E-connection skill gathering monsters
             //12070000431, // (0), [开大特殊能量]终极E2-4hit // Translated: [Open the special energy] Ultimate E2-4hit
             //12070000102, // (0), 露帕特殊能量加25-基础 // Translated: Lupa special energy plus 25-Basic
             //12070100202, // (0), 移除-强化E1-可释放 // Translated: Remove-E1-releaseable
             //12070000312, // (0), [基础特殊能量]闪避反击2 // Translated: [Basic Special Energy] Dodge Counterattack 2
             //12070000441, // (0), [开大特殊能量]冲刺技能2 // Translated: [Open the special energy] Sprint skill 2
             //12070000323, // (0), [基础特殊能量]强化重击2-2-4hit // Translated: [Basic Special Energy] Enhanced heavy blow 2-2-4hit
             //12070910202, // (0), 露帕显示火焰尾巴 // Translated: Lupa shows flame tail
             //12070000417, // (0), [开大特殊能量]空中普攻3-2 // Translated: [Open the special energy] Air normal attack 3-2
             //12070000413, // (0), [开大特殊能量]空中普攻1-挑飞 // Translated: [Open the special energy] Air normal attack 1-Choose
             //12070000335, // (0), [基础特殊能量]终结E连携技伤害 // Translated: [Basic Special Energy] End E-connection Skill Damage
             //12070000324, // (0), [基础特殊能量]强化重击2-3 // Translated: [Basic Special Energy] Enhanced heavy blow 2-3
             //12070000403, // (0), [开大特殊能量]普攻2-1 // Translated: [Open the special energy] General attack 2-1
             //12070000414, // (0), [开大特殊能量]空中普攻2-1 // Translated: [Open the special energy] Air normal attack 2-1
             //12070000411, // (0), [开大特殊能量]闪避反击1 4hit // Translated: [Open the special energy] Dodge counterattack 1 4hit
             //12070000409, // (0), [开大特殊能量]下落攻击1 // Translated: [Open the special energy] Drop attack 1
             //12070100102, // (0), 移除-猎杀模式-生效状态 // Translated: Remove-hunting mode-effective status
             //12070000404, // (0), [开大特殊能量]普攻3-1 // Translated: [Open the special energy] General attack 3-1
             //12070000408, // (0), [开大特殊能量]普攻4-3 // Translated: [Open the special energy] General attack 4-3
             //12070000402, // (0), [开大特殊能量]普攻1-3 // Translated: [Open the special energy] General attack 1-3
             //12070000416, // (0), [开大特殊能量]空中普攻3-1 // Translated: [Open the special energy] Air normal attack 3-1
             //12070000419, // (0), [开大特殊能量]基础重击1-2 // Translated: [Open the special energy] Basic heavy blow 1-2
             //12070000437, // (0), [开大特殊能量]终结E连携技普攻 // Translated: [Open the special energy] End the E-company technology general attack
             //12070000426, // (0), [开大特殊能量]基础技能1-伤害 // Translated: [Open the special energy] Basic skill 1-Damage
             //12070000306, // (0), [基础特殊能量]普攻4-1 // Translated: [Basic Special Energy] General Attack 4-1

       //1//2070900101, // (1), 基础属性50%能量监听 // Translated: Basic attribute 50% energy monitoring
       //1//2070900201, // (1), 基础属性100%能量监听 // Translated: Basic attribute 100% energy monitoring
       //1//2076120101, // (1), 可释放终结E监听被动 // Translated: Can release the end E-listener passive
       //1//2070000001, // (1), 露帕出生标识 // Translated: Lupa's birth sign
       //1//2070005001, // (1), 【共鸣5】【TAG】 // Translated: 【Resonance 5】【TAG】
       //1//2072919003, // (1), 【被动】露帕-移除烧衣服特效 // Translated: 【Pass】Lupa-Removal of burning clothes special effect
       //1//2070006001, // (1), 【共鸣6】【TAG】 // Translated: 【Resonance 6】【TAG】
       //1//2079000100, // (1), 露帕大招QTE增伤效果总控 // Translated: Lupa ultimate skill QTE damage-increasing effect total control
       //1//2070001002, // (1), 【共鸣1】触发器 // Translated: 【Resonance 1】Trigger
       //1//2070003001, // (1), 【共鸣3】【TAG】 // Translated: 【Resonance 3】【TAG】
         //1//2072919001, // (1), 满能量监听 // Translated: Full energy monitoring
         //1//2070004001, // (1), 【共鸣4】【TAG】 // Translated: 【Resonance 4】【TAG】
         //1//2072919000, // (1), 【特效】披风燃烧Buff特效挂载 // Translated: 【Special Effect】Cloak Burning Buff Special Effect Mount
         //1//2079000001, // (1), 露帕大招抗性下降触发器总控 // Translated: Lupa ultimate move Resistance reduction trigger control
         //1//2074010101, // (1), 空中攻击次数 // Translated: Number of air attacks
         //1//2071200101, // (1), 露帕旗帜关闭 // Translated: Lupa flag close
         //1//2070600102, // (1), 普通E击杀刷新普通ECD // Translated: Normal E kill refreshes normal ECD
         //1//2078020101, // (1), 冲刺技被动 // Translated: Passive sprinting skills
         //1//2070002001, // (1), 【共鸣2】【TAG】全队获得40%增伤 // Translated: 【Resonance 2】【TAG】The whole team received 40 % damage increase
         //1//2070006002, // (1), 【共鸣6】终结E-R强化、R、强化qte无视40%防御 // Translated: 【Resonance 6】End E-R Strengthening, R, Strengthening qte ignores 40% defense
         //1//2070002002, // (1), 【共鸣2】R技能或者强化重击命中时，全队获得20%增伤 // Translated: 【Resonance 2】When R skill or enhanced heavy hit hit, the whole team will get 20 % damage increase
         //1//2072919006, // (1), 【被动】露帕-披风恢复时挂火焰粒子 // Translated: 【Passive】Lupa-Hanging flame particles when the cloak is restored
         //1//2079000200, // (1), [固有技能1]施放强化重击、大招、E二段是，攻击提升 // Translated: [Inherited Skill 1] Cast a reinforced heavy blow, ultimate move, and E stages, and attack enhancement
         //1//2070001004, // (1), 【共鸣1】强化重击、空中sp抗打断等级提升1级 // Translated: 【Resonance 1】Enhance the heavy blow, the air sp anti-interruption level is increased by 1 level
         //1//2072919002, // (1), 【被动】露帕-触发休闲时，播放烧衣服特效 // Translated: 【Passive】Lupa-plays the special effect of burning clothes when triggering leisure
         //1//2070001006, // (1), 【共鸣1】强化重击、空中sp抗打断等级提升1级 // Translated: 【Resonance 1】Enhance the heavy blow, the air sp anti-interruption level is increased by 1 level
         //1//2070001001, // (1), 【共鸣1】【TAG】施放大招，暴击提升，持续10秒 // Translated: 【Resonance 1】【TAG】Cast the ultimate move, critical hit improves, lasting 10 seconds
         //1//2070001005, // (1), 【共鸣1】大招版终结E抗打断 // Translated: 【Resonance 1】End E-resistant interruption

             12070000001,  // Lupa spawn identifier  
    12070000100,  // Lupa special energy +5 (base)  
    12070000101,  // Lupa special energy +10 (base)  
    12070000102,  // Lupa special energy +25 (base)  
    12070000104,  // Lupa special energy +100 (base)  
    12070000110,  // Lupa special energy +25 (enhanced)  
    12070000111,  // Lupa special energy +50 (enhanced)  
    12070000112,  // Lupa special energy +75 (enhanced)  
    12070000201,  // Lupa special energy −50 (base)  
    12070000202,  // Lupa special energy −100 (base)  
    12070900101,  // Basic attribute 50% energy listener  
    12070900201,  // Basic attribute 100% energy listener  
    12072919001,  // Full energy monitor  
    12072919004,  // Lupa full-energy tag mount  
    12070100901,  // Accumulates advanced energy resources  
    12070800101,  // Lupa dodge passive  
    12076120001,  // End-E listener passive  
    12076120101,  // End-E release listener passive  
    12076120102,  // End-E conversion layers  
    12076120103,  // End-E release available  
    12078020101,  // Sprint skill passive  
    12078020102,  // Sprint skill passive (trigger timing)  
    12078020103,  // Speed-up status  
    12078020104,  // Sprint skill available  
    ];

    pub fn create(&mut self, buf: &mut FightBuffInformation) {
        let handle = self
            .recycled_handles
            .get_mut(&buf.handle_id)
            .and_then(|ids| ids.pop_front())
            .unwrap_or_else(|| self.next_handle.fetch_add(1, Ordering::Relaxed));

        buf.handle_id = handle;
        buf.server_id = handle;
        buf.message_id = handle as i64;

        self.active_buf_set.entry(handle).or_insert(buf.clone());
    }

    #[inline(always)]
    pub fn remove_entity_buffs(&mut self, entity_id: i64) {
        let handles = self.active_buf_set.iter()
            .filter(|(_, buff)| buff.entity_id == entity_id)
            .map(|(&handle, _)| handle)
            .collect::<Vec<_>>();
        for handle in handles {
            self.remove(handle);
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, handle: i32) -> bool {
        if let Some(buf) = self.active_buf_set.remove(&handle) {
            self.recycled_handles
                .entry(handle)
                .or_default()
                .push_back(buf.handle_id);
            true
        } else {
            false
        }
    }

    pub fn create_permanent_buffs(&mut self, origin_id: i64) -> Vec<FightBuffInformation> {
        Self::PERMANENT_ROLE_BUFFS
            .iter()
            .map(|&id| {
                let mut buff = FightBuffInformation {
                    handle_id: 0,
                    buff_id: id,
                    level: 1,
                    stack_count: 1,
                    instigator_id: origin_id,
                    entity_id: origin_id,
                    apply_type: 0,
                    duration: -1f32,
                    left_duration: -1f32,
                    context: vec![],
                    is_active: true,
                    server_id: 0,
                    message_id: 0,
                };
                self.create(&mut buff);
                buff
            })
            .collect::<Vec<_>>()
    }
}

impl Default for BufManager {
    fn default() -> Self {
        Self {
            active_buf_set: Default::default(),
            next_handle: AtomicI32::new(1),
            recycled_handles: Default::default(),
        }
    }
}
