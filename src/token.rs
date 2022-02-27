use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    // Keyword,
    // Identifier,
    // Constant,
    // Operators, //操作符
    // Special,   //特殊字符
    // Comments,  //注释

    //separator begin
    #[regex(r"[ \\t\\n\\r\\f]")]
    Space,

    #[regex(r"[\n\r]")]
    NewLine,

    #[token(r";")]
    Semicolon,

    //separator end

    //Identifier begin
    #[regex(r"`[A-Za-z_][A-Za-z_0-9\$]*`", |lex| lex.slice())]
    IdentifierWithBackQuote(&'a str),

    #[regex(r"'[A-Za-z_][A-Za-z_0-9\$]*'", |lex| lex.slice())]
    IdentifierWithQuote(&'a str),

    //Identifier begin



    //Special TODO 没做完
    #[token(r".")]
    Dot,
    

    //Comments begin
    CommentText,
    //Comments end

    //Operators begin

    //Arithmetic operator
    #[token(r"*")]
    Multiply, // *

    #[token(r"/")]
    Divide,   // /

    #[token(r"%")]
    Modulo,   // %

    #[token(r"+")]
    Plus,     // +

    #[token(r"-")]
    Minus,    // -

    //Comparison operator

    #[token(r"=")]
    Equal,              // =

    #[token(r"==")]
    Equal2,             // ==

    #[token(r"<=>")]
    NullSafeEqual,      // <=>

    #[token(r">=")]
    GreaterThanOrEqual, // >=

    #[token(r"<=")]
    LessThanOrEqual,    // <=

    #[token(r"=>")]
    EqualOrGreaterThan, // =>

    #[token(r"=<")]
    EqualOrLessThan,    // =<

    #[token(r"<>")]
    EqualWithArrows,    // <>

    #[token(r"!=")]
    NotEqual,           // !=

    #[token(r">")]
    GreaterThan,        // >

    #[token(r"<")]
    LessThan,           // <

    //Bitwise operator
    LeftShift,  // <<
    RightShift, // >>
    AndChar,        // &
    OrChar,         // |

    //Json operator
    SpecifiedPath,       // #>
    SpecifiedPathAsText, // #>>

    //Logical operator 逻辑操作符大部分再keyword中已经表述
    //Operators end

    //Keyword begin


    #[token("abort", ignore(ascii_case))]
    Abort,            //abort

    #[token("absolute", ignore(ascii_case))]
    Absolute,         //absolute

    #[token("access", ignore(ascii_case))]
    Access,           //access

    #[token("action", ignore(ascii_case))]
    Action,           //action

    #[token("add", ignore(ascii_case))]
    Add,              //add

    #[token("admin", ignore(ascii_case))]
    Admin,            //admin

    #[token("after", ignore(ascii_case))]
    After,            //after

    #[token("aggregate", ignore(ascii_case))]
    Aggregate,        //aggregate

    #[token("all", ignore(ascii_case))]
    All,              //all

    #[token("also", ignore(ascii_case))]
    Also,             //also

    #[token("alter", ignore(ascii_case))]
    Alter,            //alter

    #[token("always", ignore(ascii_case))]
    Always,           //always

    #[token("analyse", ignore(ascii_case))]
    Analyse,          //analyse

    #[token("analyze", ignore(ascii_case))]
    Analyze,          //analyze

    #[token("and", ignore(ascii_case))]
    And,              //and

    #[token("any", ignore(ascii_case))]
    Any,              //any

    #[token("array", ignore(ascii_case))]
    Array,            //array

    #[token("as", ignore(ascii_case))]
    As,               //as

    #[token("asc", ignore(ascii_case))]
    Asc,              //asc

    #[token("assertion", ignore(ascii_case))]
    Assertion,        //assertion

    #[token("assignment", ignore(ascii_case))]
    Assignment,       //assignment

    #[token("asymmetric", ignore(ascii_case))]
    Asymmetric,       //asymmetric

    #[token("at", ignore(ascii_case))]
    At,               //at

    #[token("attach", ignore(ascii_case))]
    Attach,           //attach

    #[token("attribute", ignore(ascii_case))]
    Attribute,        //attribute

    #[token("authorization", ignore(ascii_case))]
    Authorization,    //authorization

    #[token("backward", ignore(ascii_case))]
    Backward,         //backward

    #[token("before", ignore(ascii_case))]
    Before,           //before

    #[token("begin", ignore(ascii_case))]
    Begin,            //begin

    #[token("between", ignore(ascii_case))]
    Between,          //between

    #[token("bigint", ignore(ascii_case))]
    Bigint,           //bigint

    #[token("binary", ignore(ascii_case))]
    Binary,           //binary

    #[token("bit", ignore(ascii_case))]
    Bit,              //bit

    #[token("boolean", ignore(ascii_case))]
    Boolean,          //boolean

    #[token("both", ignore(ascii_case))]
    Both,             //both

    #[token("by", ignore(ascii_case))]
    By,               //by

    #[token("cache", ignore(ascii_case))]
    Cache,            //cache

    #[token("call", ignore(ascii_case))]
    Call,             //call

    #[token("called", ignore(ascii_case))]
    Called,           //called

    #[token("cascade", ignore(ascii_case))]
    Cascade,          //cascade

    #[token("cascaded", ignore(ascii_case))]
    Cascaded,         //cascaded

    #[token("case", ignore(ascii_case))]
    Case,             //case

    #[token("cast", ignore(ascii_case))]
    Cast,             //cast

    #[token("catalog", ignore(ascii_case))]
    Catalog,          //catalog

    #[token("chain", ignore(ascii_case))]
    Chain,            //chain

    #[token("char", ignore(ascii_case))]
    Char,             //char

    #[token("character", ignore(ascii_case))]
    Character,        //character

    #[token("characteristics", ignore(ascii_case))]
    Characteristics,  //characteristics

    #[token("check", ignore(ascii_case))]
    Check,            //check

    #[token("checkpoint", ignore(ascii_case))]
    Checkpoint,       //checkpoint

    #[token("class", ignore(ascii_case))]
    Class,            //class

    #[token("close", ignore(ascii_case))]
    Close,            //close

    #[token("cluster", ignore(ascii_case))]
    Cluster,          //cluster

    #[token("coalesce", ignore(ascii_case))]
    Coalesce,         //coalesce

    #[token("collate", ignore(ascii_case))]
    Collate,          //collate

    #[token("collation", ignore(ascii_case))]
    Collation,        //collation

    #[token("column", ignore(ascii_case))]
    Column,           //column

    #[token("columns", ignore(ascii_case))]
    Columns,          //columns

    #[token("comment", ignore(ascii_case))]
    Comment,          //comment

    #[token("comments", ignore(ascii_case))]
    Comments,         //comments

    #[token("commit", ignore(ascii_case))]
    Commit,           //commit

    #[token("committed", ignore(ascii_case))]
    Committed,        //committed

    #[token("concurrently", ignore(ascii_case))]
    Concurrently,     //concurrently

    #[token("configuration", ignore(ascii_case))]
    Configuration,    //configuration

    #[token("conflict", ignore(ascii_case))]
    Conflict,         //conflict

    #[token("connection", ignore(ascii_case))]
    Connection,       //connection

    #[token("constraint", ignore(ascii_case))]
    Constraint,       //constraint

    #[token("constraints", ignore(ascii_case))]
    Constraints,      //constraints

    #[token("content", ignore(ascii_case))]
    Content,          //content

    #[token("continue", ignore(ascii_case))]
    Continue,         //continue

    #[token("conversion", ignore(ascii_case))]
    Conversion,       //conversion

    #[token("copy", ignore(ascii_case))]
    Copy,             //copy

    #[token("cost", ignore(ascii_case))]
    Cost,             //cost

    #[token("create", ignore(ascii_case))]
    Create,           //create

    #[token("cross", ignore(ascii_case))]
    Cross,            //cross

    #[token("csv", ignore(ascii_case))]
    Csv,              //csv

    #[token("cube", ignore(ascii_case))]
    Cube,             //cube

    #[token("current", ignore(ascii_case))]
    Current,          //current

    #[token("current_catalog", ignore(ascii_case))]
    CurrentCatalog,   //current_catalog

    #[token("current_date", ignore(ascii_case))]
    CurrentDate,      //current_date

    #[token("current_role", ignore(ascii_case))]
    CurrentRole,      //current_role

    #[token("current_schema", ignore(ascii_case))]
    CurrentSchema,    //current_schema

    #[token("current_time", ignore(ascii_case))]
    CurrentTime,      //current_time

    #[token("current_timestamp", ignore(ascii_case))]
    CurrentTimestamp, //current_timestamp

    #[token("current_user", ignore(ascii_case))]
    CurrentUser,      //current_user

    #[token("cursor", ignore(ascii_case))]
    Cursor,           //cursor

    #[token("cycle", ignore(ascii_case))]
    Cycle,            //cycle

    #[token("data", ignore(ascii_case))]
    Data,             //data

    #[token("database", ignore(ascii_case))]
    Database,         //database

    #[token("day", ignore(ascii_case))]
    Day,              //day

    #[token("deallocate", ignore(ascii_case))]
    Deallocate,       //deallocate

    #[token("dec", ignore(ascii_case))]
    Dec,              //dec

    #[token("decimal", ignore(ascii_case))]
    Decimal,          //decimal

    #[token("declare", ignore(ascii_case))]
    Declare,          //declare

    #[token("default", ignore(ascii_case))]
    Default,          //default

    #[token("defaults", ignore(ascii_case))]
    Defaults,         //defaults

    #[token("deferrable", ignore(ascii_case))]
    Deferrable,       //deferrable

    #[token("deferred", ignore(ascii_case))]
    Deferred,         //deferred

    #[token("definer", ignore(ascii_case))]
    Definer,          //definer

    #[token("delete", ignore(ascii_case))]
    Delete,           //delete

    #[token("delimiter", ignore(ascii_case))]
    Delimiter,        //delimiter

    #[token("delimiters", ignore(ascii_case))]
    Delimiters,       //delimiters

    #[token("depends", ignore(ascii_case))]
    Depends,          //depends

    #[token("desc", ignore(ascii_case))]
    Desc,             //desc

    #[token("detach", ignore(ascii_case))]
    Detach,           //detach

    #[token("dictionary", ignore(ascii_case))]
    Dictionary,       //dictionary

    #[token("disable", ignore(ascii_case))]
    Disable,          //disable

    #[token("discard", ignore(ascii_case))]
    Discard,          //discard

    #[token("distinct", ignore(ascii_case))]
    Distinct,         //distinct

    #[token("do", ignore(ascii_case))]
    Do,               //do

    #[token("document", ignore(ascii_case))]
    Document,         //document

    #[token("domain", ignore(ascii_case))]
    Domain,           //domain

    #[token("double", ignore(ascii_case))]
    Double,           //double

    #[token("drop", ignore(ascii_case))]
    Drop,             //drop

    #[token("each", ignore(ascii_case))]
    Each,             //each

    #[token("else", ignore(ascii_case))]
    Else,             //else

    #[token("enable", ignore(ascii_case))]
    Enable,           //enable

    #[token("encoding", ignore(ascii_case))]
    Encoding,         //encoding

    #[token("encrypted", ignore(ascii_case))]
    Encrypted,        //encrypted

    #[token("end", ignore(ascii_case))]
    End,              //end

    #[token("enum", ignore(ascii_case))]
    Enum,             //enum

    #[token("escape", ignore(ascii_case))]
    Escape,           //escape

    #[token("event", ignore(ascii_case))]
    Event,            //event

    #[token("except", ignore(ascii_case))]
    Except,           //except

    #[token("exclude", ignore(ascii_case))]
    Exclude,          //exclude

    #[token("excluding", ignore(ascii_case))]
    Excluding,        //excluding

    #[token("exclusive", ignore(ascii_case))]
    Exclusive,        //exclusive

    #[token("execute", ignore(ascii_case))]
    Execute,          //execute

    #[token("exists", ignore(ascii_case))]
    Exists,           //exists

    #[token("explain", ignore(ascii_case))]
    Explain,          //explain

    #[token("expression", ignore(ascii_case))]
    Expression,       //expression

    #[token("extension", ignore(ascii_case))]
    Extension,        //extension

    #[token("external", ignore(ascii_case))]
    External,         //external

    #[token("extract", ignore(ascii_case))]
    Extract,          //extract

    #[token("false", ignore(ascii_case))]
    False,            //false

    #[token("family", ignore(ascii_case))]
    Family,           //family

    #[token("fetch", ignore(ascii_case))]
    Fetch,            //fetch

    #[token("filter", ignore(ascii_case))]
    Filter,           //filter

    #[token("first", ignore(ascii_case))]
    First,            //first

    #[token("float", ignore(ascii_case))]
    Float,            //float

    #[token("following", ignore(ascii_case))]
    Following,        //following

    #[token("for", ignore(ascii_case))]
    For,              //for

    #[token("force", ignore(ascii_case))]
    Force,            //force

    #[token("foreign", ignore(ascii_case))]
    Foreign,          //foreign

    #[token("forward", ignore(ascii_case))]
    Forward,          //forward

    #[token("freeze", ignore(ascii_case))]
    Freeze,           //freeze

    #[token("from", ignore(ascii_case))]
    From,             //from

    #[token("full", ignore(ascii_case))]
    Full,             //full

    #[token("function", ignore(ascii_case))]
    Function,         //function

    #[token("functions", ignore(ascii_case))]
    Functions,        //functions

    #[token("generated", ignore(ascii_case))]
    Generated,        //generated

    #[token("global", ignore(ascii_case))]
    Global,           //global

    #[token("grant", ignore(ascii_case))]
    Grant,            //grant

    #[token("granted", ignore(ascii_case))]
    Granted,          //granted

    #[token("greatest", ignore(ascii_case))]
    Greatest,         //greatest

    #[token("group", ignore(ascii_case))]
    Group,            //group

    #[token("grouping", ignore(ascii_case))]
    Grouping,         //grouping

    #[token("groups", ignore(ascii_case))]
    Groups,           //groups

    #[token("handler", ignore(ascii_case))]
    Handler,          //handler

    #[token("having", ignore(ascii_case))]
    Having,           //having

    #[token("header", ignore(ascii_case))]
    Header,           //header

    #[token("hold", ignore(ascii_case))]
    Hold,             //hold

    #[token("hour", ignore(ascii_case))]
    Hour,             //hour

    #[token("identity", ignore(ascii_case))]
    Identity,         //identity

    #[token("if", ignore(ascii_case))]
    If,               //if

    #[token("ilike", ignore(ascii_case))]
    Ilike,            //ilike

    #[token("immediate", ignore(ascii_case))]
    Immediate,        //immediate

    #[token("immutable", ignore(ascii_case))]
    Immutable,        //immutable

    #[token("implicit", ignore(ascii_case))]
    Implicit,         //implicit

    #[token("import", ignore(ascii_case))]
    Import,           //import

    #[token("in", ignore(ascii_case))]
    In,               //in

    #[token("include", ignore(ascii_case))]
    Include,          //include

    #[token("including", ignore(ascii_case))]
    Including,        //including

    #[token("increment", ignore(ascii_case))]
    Increment,        //increment

    #[token("index", ignore(ascii_case))]
    Index,            //index

    #[token("indexes", ignore(ascii_case))]
    Indexes,          //indexes

    #[token("inherit", ignore(ascii_case))]
    Inherit,          //inherit

    #[token("inherits", ignore(ascii_case))]
    Inherits,         //inherits

    #[token("initially", ignore(ascii_case))]
    Initially,        //initially

    #[token("inline", ignore(ascii_case))]
    Inline,           //inline

    #[token("inner", ignore(ascii_case))]
    Inner,            //inner

    #[token("inout", ignore(ascii_case))]
    Inout,            //inout

    #[token("input", ignore(ascii_case))]
    Input,            //input

    #[token("insensitive", ignore(ascii_case))]
    Insensitive,      //insensitive

    #[token("insert", ignore(ascii_case))]
    Insert,           //insert

    #[token("instead", ignore(ascii_case))]
    Instead,          //instead

    #[token("int", ignore(ascii_case))]
    Int,              //int

    #[token("integer", ignore(ascii_case))]
    Integer,          //integer

    #[token("intersect", ignore(ascii_case))]
    Intersect,        //intersect

    #[token("interval", ignore(ascii_case))]
    Interval,         //interval

    #[token("into", ignore(ascii_case))]
    Into,             //into

    #[token("invoker", ignore(ascii_case))]
    Invoker,          //invoker

    #[token("is", ignore(ascii_case))]
    Is,               //is

    #[token("isnull", ignore(ascii_case))]
    Isnull,           //isnull

    #[token("isolation", ignore(ascii_case))]
    Isolation,        //isolation

    #[token("join", ignore(ascii_case))]
    Join,             //join

    #[token("key", ignore(ascii_case))]
    Key,              //key

    #[token("label", ignore(ascii_case))]
    Label,            //label

    #[token("language", ignore(ascii_case))]
    Language,         //language

    #[token("large", ignore(ascii_case))]
    Large,            //large

    #[token("last", ignore(ascii_case))]
    Last,             //last

    #[token("lateral", ignore(ascii_case))]
    Lateral,          //lateral

    #[token("leading", ignore(ascii_case))]
    Leading,          //leading

    #[token("leakproof", ignore(ascii_case))]
    Leakproof,        //leakproof

    #[token("least", ignore(ascii_case))]
    Least,            //least

    #[token("left", ignore(ascii_case))]
    Left,             //left

    #[token("level", ignore(ascii_case))]
    Level,            //level

    #[token("like", ignore(ascii_case))]
    Like,             //like

    #[token("limit", ignore(ascii_case))]
    Limit,            //limit

    #[token("listen", ignore(ascii_case))]
    Listen,           //listen

    #[token("load", ignore(ascii_case))]
    Load,             //load

    #[token("local", ignore(ascii_case))]
    Local,            //local

    #[token("localtime", ignore(ascii_case))]
    Localtime,        //localtime

    #[token("localtimestamp", ignore(ascii_case))]
    Localtimestamp,   //localtimestamp

    #[token("location", ignore(ascii_case))]
    Location,         //location

    #[token("lock", ignore(ascii_case))]
    Lock,             //lock

    #[token("locked", ignore(ascii_case))]
    Locked,           //locked

    #[token("logged", ignore(ascii_case))]
    Logged,           //logged

    #[token("mapping", ignore(ascii_case))]
    Mapping,          //mapping

    #[token("match", ignore(ascii_case))]
    Match,            //match

    #[token("materialized", ignore(ascii_case))]
    Materialized,     //materialized

    #[token("maxvalue", ignore(ascii_case))]
    Maxvalue,         //maxvalue

    #[token("method", ignore(ascii_case))]
    Method,           //method

    #[token("minute", ignore(ascii_case))]
    Minute,           //minute

    #[token("minvalue", ignore(ascii_case))]
    Minvalue,         //minvalue

    #[token("mode", ignore(ascii_case))]
    Mode,             //mode

    #[token("month", ignore(ascii_case))]
    Month,            //month

    #[token("move", ignore(ascii_case))]
    Move,             //move

    #[token("name", ignore(ascii_case))]
    Name,             //name

    #[token("names", ignore(ascii_case))]
    Names,            //names

    #[token("national", ignore(ascii_case))]
    National,         //national

    #[token("natural", ignore(ascii_case))]
    Natural,          //natural

    #[token("nchar", ignore(ascii_case))]
    Nchar,            //nchar

    #[token("new", ignore(ascii_case))]
    New,              //new

    #[token("next", ignore(ascii_case))]
    Next,             //next

    #[token("nfc", ignore(ascii_case))]
    Nfc,              //nfc

    #[token("nfd", ignore(ascii_case))]
    Nfd,              //nfd

    #[token("nfkc", ignore(ascii_case))]
    Nfkc,             //nfkc

    #[token("nfkd", ignore(ascii_case))]
    Nfkd,             //nfkd

    #[token("no", ignore(ascii_case))]
    No,               //no

    #[token("none", ignore(ascii_case))]
    None,             //none

    #[token("normalize", ignore(ascii_case))]
    Normalize,        //normalize

    #[token("normalized", ignore(ascii_case))]
    Normalized,       //normalized

    #[token("not", ignore(ascii_case))]
    Not,              //not

    #[token("nothing", ignore(ascii_case))]
    Nothing,          //nothing

    #[token("notify", ignore(ascii_case))]
    Notify,           //notify

    #[token("notnull", ignore(ascii_case))]
    Notnull,          //notnull

    #[token("nowait", ignore(ascii_case))]
    Nowait,           //nowait

    #[token("null", ignore(ascii_case))]
    Null,             //null

    #[token("nullif", ignore(ascii_case))]
    Nullif,           //nullif

    #[token("nulls", ignore(ascii_case))]
    Nulls,            //nulls

    #[token("numeric", ignore(ascii_case))]
    Numeric,          //numeric

    #[token("object", ignore(ascii_case))]
    Object,           //object

    #[token("of", ignore(ascii_case))]
    Of,               //of

    #[token("off", ignore(ascii_case))]
    Off,              //off

    #[token("offset", ignore(ascii_case))]
    Offset,           //offset

    #[token("oids", ignore(ascii_case))]
    Oids,             //oids

    #[token("old", ignore(ascii_case))]
    Old,              //old

    #[token("on", ignore(ascii_case))]
    On,               //on

    #[token("only", ignore(ascii_case))]
    Only,             //only

    #[token("operator", ignore(ascii_case))]
    Operator,         //operator

    #[token("option", ignore(ascii_case))]
    Option,           //option

    #[token("options", ignore(ascii_case))]
    Options,          //options

    #[token("or", ignore(ascii_case))]
    Or,               //or

    #[token("order", ignore(ascii_case))]
    Order,            //order

    #[token("ordinality", ignore(ascii_case))]
    Ordinality,       //ordinality

    #[token("others", ignore(ascii_case))]
    Others,           //others

    #[token("out", ignore(ascii_case))]
    Out,              //out

    #[token("outer", ignore(ascii_case))]
    Outer,            //outer

    #[token("over", ignore(ascii_case))]
    Over,             //over

    #[token("overlaps", ignore(ascii_case))]
    Overlaps,         //overlaps

    #[token("overlay", ignore(ascii_case))]
    Overlay,          //overlay

    #[token("overriding", ignore(ascii_case))]
    Overriding,       //overriding

    #[token("owned", ignore(ascii_case))]
    Owned,            //owned

    #[token("owner", ignore(ascii_case))]
    Owner,            //owner

    #[token("parallel", ignore(ascii_case))]
    Parallel,         //parallel

    #[token("parser", ignore(ascii_case))]
    Parser,           //parser

    #[token("partial", ignore(ascii_case))]
    Partial,          //partial

    #[token("partition", ignore(ascii_case))]
    Partition,        //partition

    #[token("passing", ignore(ascii_case))]
    Passing,          //passing

    #[token("password", ignore(ascii_case))]
    Password,         //password

    #[token("placing", ignore(ascii_case))]
    Placing,          //placing

    #[token("plans", ignore(ascii_case))]
    Plans,            //plans

    #[token("policy", ignore(ascii_case))]
    Policy,           //policy

    #[token("position", ignore(ascii_case))]
    Position,         //position

    #[token("preceding", ignore(ascii_case))]
    Preceding,        //preceding

    #[token("precision", ignore(ascii_case))]
    Precision,        //precision

    #[token("prepare", ignore(ascii_case))]
    Prepare,          //prepare

    #[token("prepared", ignore(ascii_case))]
    Prepared,         //prepared

    #[token("preserve", ignore(ascii_case))]
    Preserve,         //preserve

    #[token("primary", ignore(ascii_case))]
    Primary,          //primary

    #[token("prior", ignore(ascii_case))]
    Prior,            //prior

    #[token("privileges", ignore(ascii_case))]
    Privileges,       //privileges

    #[token("procedural", ignore(ascii_case))]
    Procedural,       //procedural

    #[token("procedure", ignore(ascii_case))]
    Procedure,        //procedure

    #[token("procedures", ignore(ascii_case))]
    Procedures,       //procedures

    #[token("program", ignore(ascii_case))]
    Program,          //program

    #[token("publication", ignore(ascii_case))]
    Publication,      //publication

    #[token("quote", ignore(ascii_case))]
    Quote,            //quote

    #[token("range", ignore(ascii_case))]
    Range,            //range

    #[token("read", ignore(ascii_case))]
    Read,             //read

    #[token("real", ignore(ascii_case))]
    Real,             //real

    #[token("reassign", ignore(ascii_case))]
    Reassign,         //reassign

    #[token("recheck", ignore(ascii_case))]
    Recheck,          //recheck

    #[token("recursive", ignore(ascii_case))]
    Recursive,        //recursive

    #[token("ref", ignore(ascii_case))]
    Ref,              //ref

    #[token("references", ignore(ascii_case))]
    References,       //references

    #[token("referencing", ignore(ascii_case))]
    Referencing,      //referencing

    #[token("refresh", ignore(ascii_case))]
    Refresh,          //refresh

    #[token("reindex", ignore(ascii_case))]
    Reindex,          //reindex

    #[token("relative", ignore(ascii_case))]
    Relative,         //relative

    #[token("release", ignore(ascii_case))]
    Release,          //release

    #[token("rename", ignore(ascii_case))]
    Rename,           //rename

    #[token("repeatable", ignore(ascii_case))]
    Repeatable,       //repeatable

    #[token("replace", ignore(ascii_case))]
    Replace,          //replace

    #[token("replica", ignore(ascii_case))]
    Replica,          //replica

    #[token("reset", ignore(ascii_case))]
    Reset,            //reset

    #[token("restart", ignore(ascii_case))]
    Restart,          //restart

    #[token("restrict", ignore(ascii_case))]
    Restrict,         //restrict

    #[token("returning", ignore(ascii_case))]
    Returning,        //returning

    #[token("returns", ignore(ascii_case))]
    Returns,          //returns

    #[token("revoke", ignore(ascii_case))]
    Revoke,           //revoke

    #[token("right", ignore(ascii_case))]
    Right,            //right

    #[token("role", ignore(ascii_case))]
    Role,             //role

    #[token("rollback", ignore(ascii_case))]
    Rollback,         //rollback

    #[token("rollup", ignore(ascii_case))]
    Rollup,           //rollup

    #[token("routine", ignore(ascii_case))]
    Routine,          //routine

    #[token("routines", ignore(ascii_case))]
    Routines,         //routines

    #[token("row", ignore(ascii_case))]
    Row,              //row

    #[token("rows", ignore(ascii_case))]
    Rows,             //rows

    #[token("rule", ignore(ascii_case))]
    Rule,             //rule

    #[token("savepoint", ignore(ascii_case))]
    Savepoint,        //savepoint

    #[token("schema", ignore(ascii_case))]
    Schema,           //schema

    #[token("schemas", ignore(ascii_case))]
    Schemas,          //schemas

    #[token("scroll", ignore(ascii_case))]
    Scroll,           //scroll

    #[token("search", ignore(ascii_case))]
    Search,           //search

    #[token("second", ignore(ascii_case))]
    Second,           //second

    #[token("security", ignore(ascii_case))]
    Security,         //security

    #[token("select", ignore(ascii_case))]
    Select,           //select

    #[token("sequence", ignore(ascii_case))]
    Sequence,         //sequence

    #[token("sequences", ignore(ascii_case))]
    Sequences,        //sequences

    #[token("serializable", ignore(ascii_case))]
    Serializable,     //serializable

    #[token("server", ignore(ascii_case))]
    Server,           //server

    #[token("session", ignore(ascii_case))]
    Session,          //session

    #[token("session_user", ignore(ascii_case))]
    SessionUser,      //session_user

    #[token("set", ignore(ascii_case))]
    Set,              //set

    #[token("setof", ignore(ascii_case))]
    SetOf,            //setof

    #[token("sets", ignore(ascii_case))]
    Sets,             //sets

    #[token("share", ignore(ascii_case))]
    Share,            //share

    #[token("show", ignore(ascii_case))]
    Show,             //show

    #[token("similar", ignore(ascii_case))]
    Similar,          //similar

    #[token("simple", ignore(ascii_case))]
    Simple,           //simple

    #[token("skip", ignore(ascii_case))]
    Skip,             //skip

    #[token("smallint", ignore(ascii_case))]
    Smallint,         //smallint

    #[token("snapshot", ignore(ascii_case))]
    Snapshot,         //snapshot

    #[token("some", ignore(ascii_case))]
    Some,             //some

    #[token("sql", ignore(ascii_case))]
    Sql,              //sql

    #[token("stable", ignore(ascii_case))]
    Stable,           //stable

    #[token("standalone", ignore(ascii_case))]
    Standalone,       //standalone

    #[token("start", ignore(ascii_case))]
    Start,            //start

    #[token("statement", ignore(ascii_case))]
    Statement,        //statement

    #[token("statistics", ignore(ascii_case))]
    Statistics,       //statistics

    #[token("stdin", ignore(ascii_case))]
    Stdin,            //stdin

    #[token("stdout", ignore(ascii_case))]
    Stdout,           //stdout

    #[token("storage", ignore(ascii_case))]
    Storage,          //storage

    #[token("stored", ignore(ascii_case))]
    Stored,           //stored

    #[token("strict", ignore(ascii_case))]
    Strict,           //strict

    #[token("strip", ignore(ascii_case))]
    Strip,            //strip

    #[token("subscription", ignore(ascii_case))]
    Subscription,     //subscription

    #[token("substring", ignore(ascii_case))]
    Substring,        //substring

    #[token("support", ignore(ascii_case))]
    Support,          //support

    #[token("symmetric", ignore(ascii_case))]
    Symmetric,        //symmetric

    #[token("sysid", ignore(ascii_case))]
    Sysid,            //sysid

    #[token("system", ignore(ascii_case))]
    System,           //system

    #[token("table", ignore(ascii_case))]
    Table,            //table

    #[token("tables", ignore(ascii_case))]
    Tables,           //tables

    #[token("tablesample", ignore(ascii_case))]
    TableSample,      //tablesample

    #[token("tablespace", ignore(ascii_case))]
    Tablespace,       //tablespace

    #[token("temp", ignore(ascii_case))]
    Temp,             //temp

    #[token("template", ignore(ascii_case))]
    Template,         //template

    #[token("temporary", ignore(ascii_case))]
    Temporary,        //temporary

    #[token("text", ignore(ascii_case))]
    Text,             //text

    #[token("then", ignore(ascii_case))]
    Then,             //then

    #[token("ties", ignore(ascii_case))]
    Ties,             //ties

    #[token("time", ignore(ascii_case))]
    Time,             //time

    #[token("timestamp", ignore(ascii_case))]
    Timestamp,        //timestamp

    #[token("to", ignore(ascii_case))]
    To,               //to

    #[token("trailing", ignore(ascii_case))]
    Trailing,         //trailing

    #[token("transaction", ignore(ascii_case))]
    Transaction,      //transaction

    #[token("transform", ignore(ascii_case))]
    Transform,        //transform

    #[token("treat", ignore(ascii_case))]
    Treat,            //treat

    #[token("trigger", ignore(ascii_case))]
    Trigger,          //trigger

    #[token("trim", ignore(ascii_case))]
    Trim,             //trim

    #[token("true", ignore(ascii_case))]
    True,             //true

    #[token("truncate", ignore(ascii_case))]
    Truncate,         //truncate

    #[token("trusted", ignore(ascii_case))]
    Trusted,          //trusted

    #[token("type", ignore(ascii_case))]
    Type,             //type

    #[token("types", ignore(ascii_case))]
    Types,            //types

    #[token("uescape", ignore(ascii_case))]
    Uescape,          //uescape

    #[token("unbounded", ignore(ascii_case))]
    Unbounded,        //unbounded

    #[token("uncommitted", ignore(ascii_case))]
    Uncommitted,      //uncommitted

    #[token("unencrypted", ignore(ascii_case))]
    Unencrypted,      //unencrypted

    #[token("union", ignore(ascii_case))]
    Union,            //union

    #[token("unique", ignore(ascii_case))]
    Unique,           //unique

    #[token("unknown", ignore(ascii_case))]
    Unknown,          //unknown

    #[token("unlisten", ignore(ascii_case))]
    Unlisten,         //unlisten

    #[token("unlogged", ignore(ascii_case))]
    Unlogged,         //unlogged

    #[token("until", ignore(ascii_case))]
    Until,            //until

    #[token("update", ignore(ascii_case))]
    Update,           //update

    #[token("user", ignore(ascii_case))]
    User,             //user

    #[token("using", ignore(ascii_case))]
    Using,            //using

    #[token("vacuum", ignore(ascii_case))]
    Vacuum,           //vacuum

    #[token("valid", ignore(ascii_case))]
    Valid,            //valid

    #[token("validate", ignore(ascii_case))]
    Validate,         //validate

    #[token("validator", ignore(ascii_case))]
    Validator,        //validator

    #[token("value", ignore(ascii_case))]
    Value,            //value

    #[token("values", ignore(ascii_case))]
    Values,           //values

    #[token("varchar", ignore(ascii_case))]
    Varchar,          //varchar

    #[token("variadic", ignore(ascii_case))]
    Variadic,         //variadic

    #[token("varying", ignore(ascii_case))]
    Varying,          //varying

    #[token("verbose", ignore(ascii_case))]
    Verbose,          //verbose

    #[token("version", ignore(ascii_case))]
    Version,          //version

    #[token("view", ignore(ascii_case))]
    View,             //view

    #[token("views", ignore(ascii_case))]
    Views,            //views

    #[token("volatile", ignore(ascii_case))]
    Volatile,         //volatile

    #[token("when", ignore(ascii_case))]
    When,             //when

    #[token("where", ignore(ascii_case))]
    Where,            //where

    #[token("whitespace", ignore(ascii_case))]
    Whitespace,       //whitespace

    #[token("window", ignore(ascii_case))]
    Window,           //window

    #[token("with", ignore(ascii_case))]
    With,             //with

    #[token("within", ignore(ascii_case))]
    Within,           //within

    #[token("without", ignore(ascii_case))]
    Without,          //without

    #[token("work", ignore(ascii_case))]
    Work,             //work

    #[token("wrapper", ignore(ascii_case))]
    Wrapper,          //wrapper

    #[token("write", ignore(ascii_case))]
    Write,            //write

    #[token("xml", ignore(ascii_case))]
    Xml,              //xml

    #[token("xmlattributes", ignore(ascii_case))]
    XmlAttributes,    //xmlattributes

    #[token("xmlconcat", ignore(ascii_case))]
    XmlConcat,        //xmlconcat

    #[token("xmlelement", ignore(ascii_case))]
    XmlElement,       //xmlelement

    #[token("xmlexists", ignore(ascii_case))]
    XmlExists,        //xmlexists

    #[token("xmlforest", ignore(ascii_case))]
    XmlForest,        //xmlforest

    #[token("xmlnamespaces", ignore(ascii_case))]
    XmlNamespaces,    //xmlnamespaces

    #[token("xmlparse", ignore(ascii_case))]
    XmlParse,         //xmlparse

    #[token("xmlpi", ignore(ascii_case))]
    XmlPi,            //xmlpi

    #[token("xmlroot", ignore(ascii_case))]
    XmlRoot,          //xmlroot

    #[token("xmlserialize", ignore(ascii_case))]
    XmlSerialize,     //xmlserialize

    #[token("xmltable", ignore(ascii_case))]
    XmlTable,         //xmltable

    #[token("year", ignore(ascii_case))]
    Year,             //year

    #[token("yes", ignore(ascii_case))]
    Yes,              //yes

    #[token("zone", ignore(ascii_case))]
    Zone,             //zone

    //Keyword end

    #[error]
    Error
}
