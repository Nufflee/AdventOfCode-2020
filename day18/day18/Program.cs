using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Sprache;

public record Expression;

public enum OperatorType
{
  Add,
  Multiply
}

public record BinaryExpression(Expression Left, OperatorType Operator, Expression Right) : Expression;

public record LiteralExpression(double Value) : Expression;

public static class Program
{
  private static readonly Parser<LiteralExpression> numberParser = Parse.Digit.Many().Token().Text().Select(x => new LiteralExpression(double.Parse(x)));

  private static Parser<OperatorType> Operator(char @char, OperatorType type) => Parse.Char(@char).Token().Return(type);

  private static readonly Parser<OperatorType> addOperator = Operator('+', OperatorType.Add);
  private static readonly Parser<OperatorType> multiplyOperator = Operator('*', OperatorType.Multiply);

  private static Parser<Expression> OperandParser(Parser<Expression> expressionParser) => (
    from leftParenthesis in Parse.Char('(')
    from expression in expressionParser
    from rightParenthesis in Parse.Char(')')
    select expression
  ).Or(numberParser);

  private static BinaryExpression MakeBinaryExpression(OperatorType @operator, Expression left, Expression right) => new(left, @operator, right);

  private static readonly Parser<Expression> p1ExpressionParser =
    Parse.ChainOperator(addOperator.Or(multiplyOperator), OperandParser(Parse.Ref(() => p1ExpressionParser)), MakeBinaryExpression);

  private static readonly Parser<Expression> lowerPrecedenceParser = Parse.ChainOperator(addOperator, OperandParser(Parse.Ref(() => p2ExpressionParser)), MakeBinaryExpression);

  private static readonly Parser<Expression> p2ExpressionParser =
    Parse.ChainOperator(multiplyOperator, lowerPrecedenceParser.Or(OperandParser(Parse.Ref(() => p2ExpressionParser))), MakeBinaryExpression);

  private static double EvaluateExpression(Expression expression)
  {
    if (expression is LiteralExpression {Value: double value})
    {
      return value;
    }

    if (expression is BinaryExpression {Left: Expression left, Operator: OperatorType @operator, Right: Expression right})
    {
      double leftValue = EvaluateExpression(left);
      double rightValue = EvaluateExpression(right);

      return @operator switch
      {
        OperatorType.Add => leftValue + rightValue,
        OperatorType.Multiply => leftValue * rightValue,
        _ => throw new Exception("unreachable")
      };
    }

    throw new Exception($"Unknown Expression '{expression.GetType().Name}'");
  }

  private static double Solve(IEnumerable<string> lines, Parser<Expression> parser)
  {
    return lines.Aggregate(0.0, (acc, line) => acc + EvaluateExpression(parser.Parse(line)));
  }

  public static void Main()
  {
    string[] lines = File.ReadAllLines("test.txt");

    Console.WriteLine("Part 1:");
    Console.WriteLine($"\tSum = {Solve(lines, p1ExpressionParser)}");

    Console.WriteLine("Part 2:");
    Console.WriteLine($"\tSum = {Solve(lines, p2ExpressionParser)}");
  }
}