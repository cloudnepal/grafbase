import { RequireAtLeastOne } from 'type-fest'
import { Enum } from '../enum'
import {
  BooleanListDefinition,
  DateListDefinition,
  ListDefinition,
  NumberListDefinition,
  StringListDefinition
} from './list'
import { FieldType } from '../typedefs'
import { DefaultDefinition, DefaultValueType } from './default'
import {
  FieldLength,
  LengthLimitedStringDefinition
} from './length-limited-string'
import { SearchDefinition } from './search'
import { UniqueDefinition } from './unique'
import { AuthDefinition } from './auth'
import { AuthRuleF } from '../auth'
import { ResolverDefinition } from './resolver'
import { CacheDefinition, FieldCacheParams, FieldLevelCache } from './cache'

export class ScalarDefinition {
  private _fieldType: FieldType | Enum<any, any>
  private isOptional: boolean
  protected defaultValue?: DefaultValueType

  constructor(fieldType: FieldType | Enum<any, any>) {
    this._fieldType = fieldType
    this.isOptional = false
  }

  /**
   * The type of the field
   */
  public get fieldType(): FieldType | Enum<any, any> {
    return this._fieldType
  }

  /**
   * Make the field optional.
   */
  public optional(): this {
    this.isOptional = true

    return this
  }

  /**
   * Make the field unique.
   *
   * @param scope - Additional fields to be added to the constraint.
   */
  public unique(scope?: string[]): UniqueDefinition {
    return new UniqueDefinition(this, scope)
  }

  /**
   * Make the field searchable.
   */
  public search(): SearchDefinition {
    return new SearchDefinition(this)
  }

  /**
   * Allow multiple scalars to be used as values for the field.
   */
  public list(): ListDefinition {
    return new ListDefinition(this)
  }

  /**
   * Set the field-level auth directive.
   *
   * @param rules - A closure to build the authentication rules.
   */
  public auth(rules: AuthRuleF): AuthDefinition {
    return new AuthDefinition(this, rules)
  }

  /**
   * Attach a resolver function to the field.
   *
   * @param name - The name of the resolver function file without the extension or directory.
   */
  public resolver(name: string): ResolverDefinition {
    return new ResolverDefinition(this, name)
  }

  /**
   * Set the field-level cache directive.
   *
   * @param params - The cache definition parameters.
   */
  public cache(params: FieldCacheParams): CacheDefinition {
    return new CacheDefinition(this, new FieldLevelCache(params))
  }

  fieldTypeVal(): FieldType | Enum<any, any> {
    return this.fieldType
  }

  public toString(): string {
    const required = this.isOptional ? '' : '!'

    let fieldType
    if (this.fieldType instanceof Enum) {
      fieldType = this.fieldType.name
    } else {
      fieldType = this.fieldType.toString()
    }

    return `${fieldType}${required}`
  }
}

export class StringDefinition extends ScalarDefinition {
  /**
   * Set the default value of the field.
   *
   * @param value - The value written to the database.
   */
  public default(value: string): DefaultDefinition {
    return new DefaultDefinition(this, value)
  }

  /**
   * Specify a minimum or a maximum (or both) length of the field.
   *
   * @param fieldLength - Either `min`, `max` or both.
   */
  public length(
    fieldLength: RequireAtLeastOne<FieldLength, 'min' | 'max'>
  ): LengthLimitedStringDefinition {
    return new LengthLimitedStringDefinition(this, fieldLength)
  }

  /**
   * Specify a minimum or a maximum (or both) length of the field.
   */
  public list(): StringListDefinition {
    return new StringListDefinition(this)
  }
}

export class NumberDefinition extends ScalarDefinition {
  /**
   * Set the default value of the field.
   *
   * @param value - The value written to the database.
   */
  public default(value: number): DefaultDefinition {
    return new DefaultDefinition(this, value)
  }

  /**
   * Allow multiple scalars to be used as values for the field.
   */
  public list(): NumberListDefinition {
    return new NumberListDefinition(this)
  }
}

export class BooleanDefinition extends ScalarDefinition {
  /**
   * Set the default value of the field.
   *
   * @param value - The value written to the database.
   */
  public default(value: boolean): DefaultDefinition {
    return new DefaultDefinition(this, value)
  }

  /**
   * Allow multiple scalars to be used as values for the field.
   */
  public list(): BooleanListDefinition {
    return new BooleanListDefinition(this)
  }
}

export class DateDefinition extends ScalarDefinition {
  /**
   * Set the default value of the field.
   *
   * @param value - The value written to the database.
   */
  public default(value: Date): DefaultDefinition {
    return new DefaultDefinition(this, value)
  }

  /**
   * Allow multiple scalars to be used as values for the field.
   */
  public list(): DateListDefinition {
    return new DateListDefinition(this)
  }
}

export class ObjectDefinition extends ScalarDefinition {
  /**
   * Set the default value of the field.
   *
   * @param value - The value written to the database.
   */
  public default(value: object): DefaultDefinition {
    return new DefaultDefinition(this, value)
  }
}
